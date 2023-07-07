use yew::prelude::*;
use crate::{views::{Home, Route, reader::{SummaryView, PubViewRedirect}}, components::publication::ChapterView};

use yew_router::prelude::*;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <Home/> },
        Route::SummaryView { lang, categ, pub_symbol } => html!{ <SummaryView lang={lang} category={categ} pub_symbol={pub_symbol}/> },
        Route::PubViewRedirect { lang, categ, pub_symbol, chapter_id } => html!{ <PubViewRedirect lang={lang} category={categ} pub_symbol={pub_symbol} chapter_id={chapter_id}/> },
        Route::PubView { lang, categ, pub_symbol, chapter_id } => html!{ <ChapterView lang={lang} category={categ} pub_symbol={pub_symbol} chapter_id={chapter_id}/> },
        Route::PubFoot => html!{ <h1>{"todo!"}</h1> }
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <>
            <div class={classes!("side-navbar")}>
                <a href={"/"}>
                    <div class={classes!("nav-button")}>
                        <img src={"public/house-icon.svg"}/>
                    </div>
                </a>
            </div>
            <div class={classes!("section-area")}>
                <BrowserRouter>
                    <Switch<Route> render={switch}/>
                </BrowserRouter>
            </div>
        </>
    }
}
