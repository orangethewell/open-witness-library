use i18n_embed::{fluent::{FluentLanguageLoader, fluent_language_loader}, LanguageLoader};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use i18n_embed_fl::fl;
use crate::utils::{tauri_wrappers::get_available_categories, Localizations};

#[allow(non_snake_case)]
#[derive(Properties, PartialEq, Clone)]
pub struct CategoryButtonProps {
    pub referTo: AttrValue,
    pub onCategorySelected: Callback<String>,
}

#[function_component(CategoryButton)]
pub fn category_button(props: &CategoryButtonProps) -> Html {
    let stated_property = use_state(|| props.clone().to_owned());
    let language_loader: FluentLanguageLoader = fluent_language_loader!();
    language_loader.load_languages(&Localizations, &[language_loader.fallback_language()]);

    let category_text = props.referTo.to_string();
    let button_text = fl!(language_loader, "pub-category", category=category_text);
    let onclick = {
        let stated_property = stated_property.clone();
        Callback::from(move |_| {
            stated_property.onCategorySelected.emit(stated_property.referTo.to_string());
        })
    };
    html! {
        <button class="category-button" {onclick}>{button_text}</button>
    }
}

#[allow(non_snake_case)]
#[derive(Properties, PartialEq)]
pub struct CategoryListProps {
    pub lang: AttrValue,
    pub onCategorySelected: Callback<String>,
}

#[function_component(CategoryList)]
pub fn category_list(props: &CategoryListProps) -> Html {
    let categories = use_state(|| {let vecstring: Vec<String> = vec![]; vecstring});
    let lang = props.lang.clone().to_string();

    {
        let cat_pointer = categories.clone();
        use_effect_with_deps(move |_| {
            let cat_pointer = cat_pointer.clone();
            spawn_local(async move {
                cat_pointer.set(get_available_categories(lang).await);
            })
        },
        ());
    }

    let categorylist = (*categories).clone();
    let category_html = categorylist.into_iter().map(|category| html!{<CategoryButton referTo={category.clone()} onCategorySelected={&props.onCategorySelected} />}).collect::<Html>();

    html!{
        <>
        {category_html}
        </>
    }
}
