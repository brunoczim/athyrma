use core::fmt;
use gardiz::{axis::Axis, coord::Vec2};
use std::{cell::Cell, error::Error, num::ParseIntError, rc::Rc};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlCanvasElement, HtmlElement, UiEvent, Window};
use webio::{
    callback::Cancelled,
    event::{self, EventType},
};

pub type IntCoord = u32;

pub type FracCoord = f64;

#[derive(Debug, Clone, Copy)]
pub enum ResizeStrategy {
    NoResize,
    KeepSize,
    KeepRatio,
    AdaptRatio,
}

impl Default for ResizeStrategy {
    fn default() -> Self {
        Self::KeepRatio
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    inner: Rc<CanvasInner>,
    events: Rc<CanvasEvents>,
}

impl Canvas {
    pub fn new(parent: HtmlElement) -> Self {
        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let element = document
            .create_element("canvas")
            .unwrap_throw()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap_throw();

        parent.append_with_node_1(&element).unwrap_throw();

        let default_size = Vec2 { x: 400, y: 300 };
        let inner = Rc::new(CanvasInner {
            logical_size: Cell::new(default_size),
            display_size: Cell::new(default_size),
            parent,
            element,
            window: window.clone(),
            resize_strategy: Cell::new(ResizeStrategy::default()),
        });

        inner.resize();
        let on_window_resize = {
            let inner = inner.clone();
            event::WindowResize.add_listener_with_sync_cb(&window, move |evt| {
                inner.resize();
                evt
            })
        };

        let events = Rc::new(CanvasEvents { on_window_resize });

        Self { inner, events }
    }

    pub fn logical_size(&self) -> Vec2<IntCoord> {
        self.inner.logical_size()
    }

    pub fn set_logical_size(&self, size: Vec2<IntCoord>) {
        self.inner.set_logical_size(size);
    }

    pub fn display_size(&self) -> Vec2<IntCoord> {
        self.inner.display_size()
    }

    pub fn set_display_size(&self, size: Vec2<IntCoord>) {
        self.inner.set_display_size(size)
    }

    pub fn resize_strategy(&self) -> ResizeStrategy {
        self.inner.resize_strategy()
    }

    pub fn set_resize_strategy(&self, resize_strategy: ResizeStrategy) {
        self.inner.set_resize_strategy(resize_strategy);
    }

    pub fn force_display_size(&self, size: Vec2<IntCoord>) {
        self.inner.force_display_size(size);
    }

    pub fn translate_coord(&self, axis: Axis, input: IntCoord) -> FracCoord {
        self.inner.translate_coord(axis, input)
    }

    pub fn translate_vec2(&self, input: Vec2<IntCoord>) -> Vec2<FracCoord> {
        self.inner.translate_vec2(input)
    }

    pub async fn on_resize(&self) -> Result<UiEvent, Cancelled> {
        self.events.on_window_resize.listen_next().await
    }
}

#[derive(Debug)]
struct CanvasInner {
    logical_size: Cell<Vec2<IntCoord>>,
    display_size: Cell<Vec2<IntCoord>>,
    resize_strategy: Cell<ResizeStrategy>,
    parent: HtmlElement,
    element: HtmlCanvasElement,
    window: Window,
}

impl CanvasInner {
    fn logical_size(&self) -> Vec2<IntCoord> {
        self.logical_size.get()
    }

    fn set_logical_size(&self, size: Vec2<IntCoord>) {
        self.logical_size.set(size)
    }

    fn display_size(&self) -> Vec2<IntCoord> {
        self.display_size.get()
    }

    fn set_display_size(&self, size: Vec2<IntCoord>) {
        self.display_size.set(size);
        self.element.set_width(size.x);
        self.element.set_height(size.y);
    }

    fn resize_strategy(&self) -> ResizeStrategy {
        self.resize_strategy.get()
    }

    fn set_resize_strategy(&self, resize_strategy: ResizeStrategy) {
        self.resize_strategy.set(resize_strategy);
    }

    fn force_display_size(&self, size: Vec2<IntCoord>) {
        self.set_resize_strategy(ResizeStrategy::NoResize);
        self.set_display_size(size);
    }

    fn translate_coord(&self, axis: Axis, input: IntCoord) -> FracCoord {
        let display_size = self.display_size();
        let logical_size = self.logical_size();
        FracCoord::from(display_size[axis])
            / FracCoord::from(logical_size[axis])
            * FracCoord::from(input)
    }

    fn translate_vec2(&self, input: Vec2<IntCoord>) -> Vec2<FracCoord> {
        input.map_with_axes(|axis, coord| self.translate_coord(axis, coord))
    }

    fn parent_display_size(&self) -> Vec2<IntCoord> {
        let mut size = Vec2 {
            x: u32::try_from(self.parent.client_width()).unwrap_throw(),
            y: u32::try_from(self.parent.client_height()).unwrap_throw(),
        };

        if let Some(style) =
            self.window.get_computed_style(&self.parent).unwrap_throw()
        {
            let padding_top =
                &style.get_property_value("padding-top").unwrap_throw();
            let padding_left =
                &style.get_property_value("padding-left").unwrap_throw();
            let padding_bottom =
                &style.get_property_value("padding-bottom").unwrap_throw();
            let padding_right =
                &style.get_property_value("padding-right").unwrap_throw();

            size.x -= parse_computed_px(&padding_left).unwrap_throw();
            size.x -= parse_computed_px(&padding_right).unwrap_throw();
            size.y -= parse_computed_px(&padding_top).unwrap_throw();
            size.y -= parse_computed_px(&padding_bottom).unwrap_throw();
        }

        size
    }

    fn resize(&self) {
        match self.resize_strategy() {
            ResizeStrategy::NoResize => (),

            ResizeStrategy::KeepSize => {
                self.set_display_size(self.logical_size());
            },

            ResizeStrategy::KeepRatio => {
                let parent_display_size = self.parent_display_size();
                let logical_size = self.logical_size();

                // lx / ly <= px / py
                //
                // i.e. is logical less horizontal than parent display?
                let parent_logical_rel = Vec2 {
                    x: parent_display_size.y * logical_size.x,
                    y: parent_display_size.x * logical_size.y,
                };
                if parent_logical_rel.x <= parent_logical_rel.y {
                    self.set_display_size(Vec2 {
                        x: parent_logical_rel.x / logical_size.y,
                        y: parent_display_size.y,
                    });
                } else {
                    self.set_display_size(Vec2 {
                        x: parent_display_size.x,
                        y: parent_logical_rel.y / logical_size.x,
                    });
                }
            },

            ResizeStrategy::AdaptRatio => {
                self.set_display_size(self.parent_display_size());
            },
        }
    }
}

#[derive(Debug)]
struct CanvasEvents {
    on_window_resize: event::Listener<UiEvent>,
}

#[derive(Debug, Clone)]
pub enum ParseComputedPxError {
    IntParseError(ParseIntError),
    TrailingChars,
    MissingPx,
}

impl fmt::Display for ParseComputedPxError {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntParseError(error) => write!(fmtr, "{}", error),
            Self::TrailingChars => write!(fmtr, "trailing characters after px"),
            Self::MissingPx => write!(fmtr, "missing px"),
        }
    }
}

impl Error for ParseComputedPxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Self::IntParseError(error) = self {
            Some(error)
        } else {
            None
        }
    }
}

fn parse_computed_px(input: &str) -> Result<IntCoord, ParseComputedPxError> {
    match input.split_once("px") {
        Some((head, "")) => {
            head.parse().map_err(ParseComputedPxError::IntParseError)
        },
        Some(_) => Err(ParseComputedPxError::TrailingChars),
        None => Err(ParseComputedPxError::MissingPx),
    }
}
