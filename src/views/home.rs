use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde_wasm_bindgen::{to_value, from_value};
use yew_router::prelude::*;
use crate::views::Route;

use crate::components::publication::PubCatalog;
use crate::utils::{
    pub_utils::Publication,
    TauriWrappers::get_list_from_category,
    log,
};


#[function_component(Home)]
pub fn home() -> Html {
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
            <button {onclick}>{ "Atualizar lista" }</button>
            <PubCatalog publications={(*publications).clone()}/>
        </>
    }
}
