#![allow(unused_must_use)]

pub mod home;
pub mod reader;
pub mod utils {
    use yew_router::prelude::*;
    #[derive(Debug, Clone, PartialEq, Routable)]
    pub enum Route {
        #[at("/")]
        Home,

        #[at("/view/:lang/:categ/:pub_symbol")]
        SummaryView {lang: String, categ: String, pub_symbol: String},

        #[at("/redirect/:lang/:categ/:pub_symbol/:chapter_id")]
        PubViewRedirect {lang: String, categ: String, pub_symbol: String, chapter_id: i32},

        #[at("/view/:lang/:categ/:pub_symbol/:chapter_id")]
        PubView {lang: String, categ: String, pub_symbol: String, chapter_id: i32},

        #[at("/view/footnotes")]
        PubFoot,
    }
}

pub use utils::Route;
pub use home::Home;
pub use reader::{PubViewRedirect, SummaryView};