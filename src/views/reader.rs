use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::Redirect;
use crate::components::publication::{PubSummary, ViewerProps};
use crate::utils::{tauri_wrappers::get_summary_from, pub_utils::Chapter};
use crate::views::Route;

#[derive(Properties, PartialEq)]
pub struct SummaryViewProps {
    pub lang: AttrValue,
    pub category: AttrValue,
    pub pub_symbol: AttrValue
}

pub struct SummaryViewAttr {
    pub lang: String,
    pub category: String,
    pub pub_symbol: String,
}

#[function_component(SummaryView)]
pub fn summary_viewer(props: &SummaryViewProps) -> Html {
    let chapter_list = use_state(|| Vec::<Chapter>::new());
    let props_stateble = use_state(|| SummaryViewAttr {
        lang: props.lang.clone().to_string(),
        category: props.category.clone().to_string(),
        pub_symbol: props.pub_symbol.clone().to_string(),
    });
    {
        let chapter_list = chapter_list.clone();
        use_effect_with_deps(move |_| {
            let props = props_stateble.clone();
            spawn_local(async move {
                chapter_list.set(get_summary_from(&props.lang, &props.category, &props.pub_symbol).await)
            });
            ||()
        }, ())
    }
    
    let output = use_memo(|_|{
        html!{
            <PubSummary chapters={(*chapter_list).clone()} lang={props.lang.clone()} category={props.category.clone()} pub_symbol={props.pub_symbol.clone()}/>
        }
    }, chapter_list.clone());

    (*output).clone()
    
}

#[function_component(PubViewRedirect)]
pub fn pub_viewer(props: &ViewerProps) -> Html {
    html! {
        <Redirect<Route> to={Route::PubView{lang: (&props).lang.clone().to_string(), categ: (&props).category.clone().to_string(), pub_symbol: (&props).pub_symbol.clone().to_string(), chapter_id: props.chapter_id as i32}}/>
    }
}