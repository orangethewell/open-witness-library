use crate::utils::{
    pub_utils::{Publication, Chapter},
    tauri_wrappers::{set_media_location, get_chapter_content},
    convert_file_src, log
};
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use yew::{prelude::*, virtual_dom::AttrValue};
use yew_router::prelude::*;
use gloo::utils::document;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Node, DomParser, SupportedType};
use crate::views::Route;
pub struct PubCatalog {
    pub limit: usize,
    pub loaded_idx: usize 
}

pub enum CatalogMsg {
    Update,
    Remove(String)
}

#[derive(Properties, PartialEq)]
pub struct CatalogProps {
    pub publications: Vec<Publication>,
}


impl Component for PubCatalog {
    type Message = CatalogMsg;
    type Properties = CatalogProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {limit: 0, loaded_idx: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <ul class={"catalog"}>
                {for ctx.props().publications.iter().map(|publication| self.render_publication(&ctx, publication))} 
            </ul>
        }
    }
}

impl PubCatalog {
    /// Show information for every publication received by Component context
    fn render_publication(&self, ctx: &Context<PubCatalog>, publication: &Publication) -> Html {
        let resolved_cover_path = convert_file_src((&publication).cover_icon_path.clone().to_str().unwrap(), None);
        let navigator: Navigator = ctx.link().navigator().unwrap();


        let onclick = {
            let publi = publication.clone();
            let navigator = navigator.clone();            

            Callback::from(move |_| {
                let mut pub_symbol = publi.symbol.to_owned();
                pub_symbol.push_str(&format!("_{}", publi.language)); // TODO: remove this plz;
                set_media_location(&publi.language, &publi.category, &pub_symbol);
            
                navigator.push(&Route::SummaryView{lang: publi.language.clone(), categ: publi.category.clone(), pub_symbol: pub_symbol.clone()});
            })
        };
        
        html!{
            <li>
                <a {onclick}>
                    <div class={"catalog-item"}>
                    <img src={resolved_cover_path}/>
                    <div class={"pub-info"}>
                        <h3>{(&publication).title.clone()}</h3>
                        <span class={"year-issue"}>{(&publication).year}</span>
                    </div>
                    </div>
                </a>
            </li>
        }
    }
}

/// Chapter list for certain Publication
#[derive(Clone)]
pub struct PubSummary{}

#[derive(Properties, PartialEq, Clone)]
pub struct SummaryProps {
    pub chapters: Vec<Chapter>,
    pub lang: AttrValue,
    pub category: AttrValue,
    pub pub_symbol: AttrValue
}

impl Component for PubSummary {
    type Message = ();
    type Properties = SummaryProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        html!{
            if props.chapters.len() > 0 {
            <ul class={"article-content"}>
                {for props.chapters.iter().map(|chapter| self.render_chapter_item(&ctx, chapter))} 
            </ul>
            } else {
                <div class={"article-content"}>
                    <p>{ "Please, Wait..." }</p>
                </div>
            }
       }
    }
}

impl PubSummary {
    pub fn render_chapter_item(&self, ctx: &Context<Self>, chapter: &Chapter) -> Html {
        let props = ctx.props().clone();
        let navigator = ctx.link().navigator().unwrap();
        let onclick = {
            let props = props.clone();
            let chapter = chapter.clone();
            Callback::from(move |_| {
                let pub_symbol = (&props).pub_symbol.clone().to_string();

                navigator.push(&Route::PubView{lang: (&props).lang.clone().to_string(), categ: (&props).category.clone().to_string(), pub_symbol: pub_symbol.clone().to_string(), chapter_id: chapter.id.clone() as i32});
            })
        };
        html!{
            <li>
                <a {onclick}>
                    <div>
                        <p>{(&chapter).title.clone()}</p>
                    </div>
                </a>
            </li>
        }
    }
}

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
#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct ChapterContent {
    content : String,
    next_exists: bool,
    previous_exists: bool,
}

#[function_component(ChapterView)]
pub fn chapter_viewer(props: &ViewerProps) -> Html {
    let content = use_state(|| ChapterContent::default());
    let props_stateble = use_state(|| ViewerAttr {
        lang: props.lang.clone().to_string(),
        category: props.category.clone().to_string(),
        pub_symbol: props.pub_symbol.clone().to_string(),
        chapter_id: props.chapter_id.clone()
    });

    {
        let content = content.clone();
        use_effect_with_deps(move |_| {
            let props_clone = props_stateble.clone();
            spawn_local(async move {
                let data = get_chapter_content(&props_clone.lang, &props_clone.category, &props_clone.pub_symbol, props_clone.chapter_id).await;    
                content.set(data);           
            });

            ||{}
        }, props.chapter_id);
    }

    let node = use_memo(
        |_|{
            log("Content changed!");
            let content = content.clone();
            let div: Element = document().create_element("div").unwrap();
            let parsed_content = DomParser::new().unwrap().parse_from_string(&*content.content, SupportedType::TextHtml).unwrap();
            let elements = parsed_content.get_elements_by_tag_name("a");
            for element in 0..elements.length(){
                if let Some(anchor_tag) = elements.item(element).unwrap().dyn_ref::<web_sys::HtmlAnchorElement>() {
                    log("found a anchor");
                    if anchor_tag.hostname().contains("jw.org") {
                        log("changing anchor target...");
                        anchor_tag.set_target("_blank");
                    }
                }
            }
            div.set_inner_html(&parsed_content.document_element().unwrap().inner_html());
            let node: Node = div.into();
            Html::VRef(node)
        },
        (content.clone(), props.chapter_id)
    );
    
    html!{
    <div class={"article-content"}>
        if content.previous_exists {
            <Link<Route> to={Route::PubViewRedirect{lang: (&props).lang.clone().to_string(), categ: (&props).category.clone().to_string(), pub_symbol: (&props).pub_symbol.clone().to_string(), chapter_id: (props.chapter_id - 1) as i32}}>
            <div class={classes!("go-route-pub", "go-pub-back")}>{"<"}</div>
            </Link<Route>>
        }
        if content.next_exists {
            <Link<Route> to={Route::PubViewRedirect{lang: (&props).lang.clone().to_string(), categ: (&props).category.clone().to_string(), pub_symbol: (&props).pub_symbol.clone().to_string(), chapter_id: (props.chapter_id + 1) as i32}}>
            <div class={classes!("go-route-pub", "go-pub-next")}>{">"}</div>
            </Link<Route>>
        }
        {(*node).clone()}
    </div>
    }
}
