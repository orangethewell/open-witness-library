use crate::utils::{
    pub_utils::{Publication, Chapter},
    TauriWrappers::set_media_location,
    convert_file_src
};
use yew::{prelude::*, virtual_dom::AttrValue};
use yew_router::prelude::*;
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
            <ul>
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
                    <div>
                    <img src={resolved_cover_path}/>
                    <p>{(&publication).title.clone()}</p>
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
            <ul>
                {for props.chapters.iter().map(|chapter| self.render_chapter_item(&ctx, chapter))} 
            </ul>
            } else {
                <div>
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