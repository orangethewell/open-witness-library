use std::rc::Rc;
use web_sys::{Element, Node};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo::utils::document;
use crate::components::publication::PubSummary;
use crate::utils::{TauriWrappers::{get_chapter_content, get_summary_from}, log, pub_utils::Chapter};

#[derive(Properties, PartialEq)]
pub struct ViewerProps {
    pub lang: AttrValue,
    pub category: AttrValue,
    pub pub_symbol: AttrValue,
    pub chapter_id: i32
}

#[derive(PartialEq)]
pub struct ViewerAttr {
    pub lang: String,
    pub category: String,
    pub pub_symbol: String,
    pub chapter_id: i32
}

pub struct PubReader {
    content: Rc<String>,
}

/// reducer's Action
enum ReaderAction {
    Fill{data: String},
}

/// reducer's State
#[derive(PartialEq)]
struct ReaderState {
    data: String,
}

impl Default for ReaderState {
    fn default() -> Self {
        Self { data: String::from("No material was read.") }
    }
}

impl Reducible for ReaderState {
    /// Reducer Action Type
    type Action = ReaderAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            ReaderAction::Fill { data } => data.clone(),
        };

        Self { data: next_ctr }.into()
    }
}


#[function_component(ChapterView)]
pub fn chapter_viewer(props: &ViewerProps) -> Html {
    let content = use_reducer(ReaderState::default);
    let prop_stateble = use_state(|| ViewerAttr {
        lang: props.lang.clone().to_string(),
        category: props.category.clone().to_string(),
        pub_symbol: props.pub_symbol.clone().to_string(),
        chapter_id: props.chapter_id.clone()
    });

    {
        let content = content.clone();
        use_effect_with_deps(move |_| {
            let props = prop_stateble.clone();
            spawn_local(async move {
                let data =  get_chapter_content(&props.lang, &props.category, &props.pub_symbol, props.chapter_id).await;
                log(&format!("yey: {}", &data));     
                content.dispatch(ReaderAction::Fill{data});           
            });

            ||{}
        }, ());
    }

    let node = use_memo(
        |_|{
            let content = content.clone();
            let div: Element = document().create_element("div").unwrap();
            div.set_inner_html(&format!("Conteúdo: {}", &content.data));
            let node: Node = div.into();
            Html::VRef(node)
        },
        content.clone()
    );

    (*node).clone()

}

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
pub fn index_viewer(props: &SummaryViewProps) -> Html {
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

#[function_component(PubView)]
pub fn pub_viewer(props: &ViewerProps) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <p>{"Controles"}</p>
            <ChapterView lang={props.lang.clone()} category={props.category.clone()} pub_symbol={props.pub_symbol.clone()} chapter_id={props.chapter_id}/>
        </Suspense>
    }
}