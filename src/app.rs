use serde_wasm_bindgen::to_value;
use yew::prelude::*;
use crate::{views::{Home, PubView, Route, reader::SummaryView}, utils::TauriWrappers::init_catalog};

use yew_router::prelude::*;

// TODO: Comere 

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <Home/> },
        Route::SummaryView { lang, categ, pub_symbol } => html!{ <SummaryView lang={lang} category={categ} pub_symbol={pub_symbol}/> },
        Route::PubView { lang, categ, pub_symbol, chapter_id } => html!{ <PubView lang={lang} category={categ} pub_symbol={pub_symbol} chapter_id={chapter_id}/> },
        Route::PubFoot => html!{ <h1>{"todo!"}</h1> }
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    init_catalog();
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
