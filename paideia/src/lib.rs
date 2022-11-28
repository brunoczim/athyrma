//! This tool allows you to generate static sites in encyclopedia-like format,
//! using Rust code. With this tool, you'll deal with "components" of web pages.
//! No JavaScript dependency required for the build output.
//!
//! # Example
//!
//! ```rust
//! use katalogos::harray;
//! use paideia::{
//!     component::{
//!         asset::Stylesheet,
//!         block::{list::UnorderedList, text::Paragraph, InlineBlock},
//!         inline::text::Link,
//!         page::{Page, PageComponent},
//!         section::Section,
//!     },
//!     location::{Id, InternalPath, Location},
//!     render::{DynFullComponent, FullRender, Html},
//!     site::{Entry, Site},
//! };
//! use std::{path::PathBuf, process};
//!
//! fn index() -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static
//! {
//!     Page {
//!         title: String::from("Simple Pedia"),
//!         assets: [Stylesheet {
//!             location: Location::internal("styles/main.css"),
//!         }],
//!         body: harray![
//!             Paragraph(
//!                 "This is the initial page of the simple pedia. You can \
//!                  dive down into the following:"
//!             ),
//!             UnorderedList(harray![
//!                 InlineBlock(Link {
//!                     location: Location::internal("foo"),
//!                     target: "Foo stuff",
//!                 }),
//!                 InlineBlock(Link {
//!                     location: Location::internal("bar"),
//!                     target: "Bar stiff",
//!                 }),
//!             ]),
//!         ],
//!         children: harray![
//!             Section {
//!                 title: "Random Section",
//!                 id: Some(Id::new("random").unwrap()),
//!                 body: Paragraph("This is a random paragraph."),
//!                 children: harray![
//!                     Section {
//!                         title: "Randomly First",
//!                         id: Some(Id::new("random-first").unwrap()),
//!                         body: Paragraph(
//!                             "This the first (really?) random paragraph."
//!                         ),
//!                         children: harray![],
//!                     },
//!                     Section {
//!                         title: "Randomly Second",
//!                         id: Some(Id::new("random-second").unwrap()),
//!                         body: Paragraph(
//!                             "This the second (really??) random paragraph."
//!                         ),
//!                         children: harray![],
//!                     }
//!                 ],
//!             },
//!             Section {
//!                 title: "Weird Title",
//!                 id: Some(Id::new("weird").unwrap()),
//!                 body: Paragraph("Weird paragraph as an example"),
//!                 children: harray![],
//!             }
//!         ],
//!     }
//! }
//!
//! fn foo_page(
//! ) -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static {
//!     Page {
//!         title: String::from("Foo"),
//!         assets: [Stylesheet {
//!             location: Location::internal("styles/main.css"),
//!         }],
//!         body: harray![Paragraph("Foo is a metavariable."),],
//!         children: harray![],
//!     }
//! }
//!
//! fn bar_page(
//! ) -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static {
//!     Page {
//!         title: String::from("Bar"),
//!         assets: [Stylesheet {
//!             location: Location::internal("styles/main.css"),
//!         }],
//!         body: harray![Paragraph(harray![
//!             "Bar is a metavariable. ",
//!             Link { location: Location::internal("bar/baz"), target: "Baz" },
//!             " is also a metavariable."
//!         ])],
//!         children: harray![],
//!     }
//! }
//!
//! fn baz_page(
//! ) -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static {
//!     Page {
//!         title: String::from("Baz"),
//!         assets: [Stylesheet {
//!             location: Location::internal("styles/main.css"),
//!         }],
//!         body: harray![Paragraph(harray![
//!             "Baz is a metavariable, similar to ",
//!             Link { location: Location::internal("bar"), target: "Bar" },
//!             "."
//!         ])],
//!         children: harray![],
//!     }
//! }
//!
//! fn simple_pedia_site() -> Site<DynFullComponent<'static, PageComponent>> {
//!     let mut site = Site::default();
//!     site.root.insert_path(
//!         &InternalPath::parse("index.html").unwrap(),
//!         Entry::Page(index().into_dyn()),
//!     );
//!     site.root.insert_path(
//!         &InternalPath::parse("foo/index.html").unwrap(),
//!         Entry::Page(foo_page().into_dyn()),
//!     );
//!     site.root.insert_path(
//!         &InternalPath::parse("bar/index.html").unwrap(),
//!         Entry::Page(bar_page().into_dyn()),
//!     );
//!     site.root.insert_path(
//!         &InternalPath::parse("bar/baz/index.html").unwrap(),
//!         Entry::Page(baz_page().into_dyn()),
//!     );
//!     site.root.insert_path(
//!         &InternalPath::parse("styles/main.css").unwrap(),
//!         Entry::Resource,
//!     );
//!     site
//! }
//!
//! fn main() {
//! # if false {
//!     let site = simple_pedia_site();
//!
//!     let result = site.build(
//!         &mut Html,
//!         &mut PathBuf::from("paideia/examples/build"),
//!         &mut PathBuf::from("paideia/examples/assets"),
//!     );
//!
//!     if let Err(error) = result {
//!         eprintln!("{}", error);
//!         process::exit(1);
//!     }
//! }
//! # }
//! ```

pub mod render;
pub mod component;
pub mod location;
pub mod site;
