use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::{publication::PubCatalog, category::CategoryList};
use crate::utils::tauri_wrappers::install_jwpub_file;
use crate::utils::{
    pub_utils::Publication,
    tauri_wrappers::get_list_from_category,
    Localizations
};

use i18n_embed::{ // TODO: Implement Tauri Language Requester
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use i18n_embed_fl::fl;

#[function_component(Home)]
pub fn home() -> Html {
    let language_loader: FluentLanguageLoader = fluent_language_loader!();
    
    language_loader.load_languages(&Localizations, &[language_loader.fallback_language()]);

    let publications = use_state(|| {
        let pub_list: Vec<Publication> = vec![];
        pub_list
    });

    let add_onclick = {
       Callback::from(move |_| {
            spawn_local(async move {
                install_jwpub_file().await;
            })
        }) 
    };

    let onCategorySelected = {
        let publications = publications.clone();
        Callback::from(move |category|  
            {
                let publications = publications.clone();
                spawn_local(async move {
                publications.set(get_list_from_category("T".to_string(), category, 0, 25).await);
                });
            }) 
        };

    html! {
        <>
            <h1>{fl!(language_loader, "welcome-library")}</h1>
            <button onclick={add_onclick}>{"+"}</button>
            <CategoryList lang="T" {onCategorySelected}/>
            <PubCatalog publications={(*publications).clone()}/>
        </>
    }
}
