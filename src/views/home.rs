use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::publication::PubCatalog;
use crate::utils::{
    pub_utils::Publication,
    TauriWrappers::get_list_from_category,
    log,
    Localizations
};

use i18n_embed::{
    WebLanguageRequester, // TODO: Implement Tauri Language Requester
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
    let onclick = {
        let publications = publications.clone();
        Callback::from(move |_|  
            {
                let publications = publications.clone();
                spawn_local(async move {
                publications.set(get_list_from_category("T".to_string(), "bk".to_string(), 0, 25).await);
                });
            }) 
        };

    html! {
        <>
            <button {onclick}>{ fl!(language_loader, "update-list") }</button>
            <PubCatalog publications={(*publications).clone()}/>
        </>
    }
}
