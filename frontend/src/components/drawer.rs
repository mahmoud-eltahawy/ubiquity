use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{contexts::config::use_config, pages::Page};

#[derive(Debug, PartialEq, Properties)]
pub struct DrawerProps {
    pub children: Children,
}

pub mod leptos_version {
    use config::Config;
    use leptos::prelude::*;
    use leptos_router::{hooks::use_navigate, NavigateOptions};

    #[component]
    pub fn Drawer(children: Children) -> impl IntoView {
        let conf = use_context::<RwSignal<Config>>().unwrap();
        let theme = move || conf.get().theme;

        view! {
            <div data-theme=theme class="drawer print:hidden">
                <input id="drawer-input" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content">
                    { children() }
                </div>
                <div class="drawer-side">
                    <label for="drawer-input" class="drawer-overlay"></label>
                    <div class="flex flex-col h-full bg-base-300 py-2 3xl:w-[10%] 2xl:w-[15%] xl:w-[20%] lg:w-[30%] md:w-[40%] sm:w-[50%] xs:w-[60%] w-[60%]">
                    <h1 class="mt-2 mb-3 text-2xl font-display font-bold tracking-wide self-center">{"Ubiquity"}</h1>
                    <div class="h-full flex flex-col justify-between">
                        <ul class="menu menu-lg bg-base-200 w-full">
                            <Home />
                            <Settings />
                        </ul>
                    </div>
                </div>
                </div>
            </div>
        }
    }

    #[component]
    pub fn Home() -> impl IntoView {
        let nav = use_navigate();
        let home = move |_| nav("/", NavigateOptions::default());

        view! {
            <li>
                <div on:click=home>
                    <svg
                        xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 fill-base-200 stroke-base-content stroke-[1.5px]"
                        viewBox="0 0 24 24"
                    >
                        <path
                            d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                        />
                    </svg>
                    "Home"
                </div>
            </li>
        }
    }

    #[component]
    pub fn Settings() -> impl IntoView {
        let nav = use_navigate();
        let settings = move |_| nav("/settings", NavigateOptions::default());

        view! {
            <li>
                <div on:click={settings}>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 fill-base-200 stroke-base-content stroke-[1.5px]" viewBox="0 0 24 24">
                    <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>
                </svg>
                    {"Settings"}
                </div>
            </li>
        }
    }
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let theme = use_config().state().theme;

    let drawer_classes = classes!(
        "flex",
        "flex-col",
        "h-full",
        "bg-base-300",
        "py-2",
        "3xl:w-[10%]",
        "2xl:w-[15%]",
        "xl:w-[20%]",
        "lg:w-[30%]",
        "md:w-[40%]",
        "sm:w-[50%]",
        "xs:w-[60%]",
        "w-[60%]",
    );

    html! {
        <div data-theme={theme} class="drawer print:hidden">
            <input id="drawer-input" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                { props.children.clone() }
            </div>
            <div class="drawer-side">
                <label for="drawer-input" class="drawer-overlay"></label>
                <div class={drawer_classes}>
                <h1 class="mt-2 mb-3 text-2xl font-display font-bold tracking-wide self-center">{"Ubiquity"}</h1>
                <div class="h-full flex flex-col justify-between">
                    <ul class="menu menu-lg bg-base-200 w-full">
                        <Home />
                        <Settings />
                    </ul>
                </div>
            </div>
            </div>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let nav = use_navigator().unwrap();
    let home = Callback::from(move |_| nav.replace(&Page::Home));

    html! {
        <li>
            <div onclick={home}>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 fill-base-200 stroke-base-content stroke-[1.5px]"
                    viewBox="0 0 24 24">
                    <path
                        d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                </svg>
                {"Home"}
            </div>
        </li>
    }
}

#[function_component(Settings)]
pub fn home() -> Html {
    let nav = use_navigator().unwrap();
    let settings = Callback::from(move |_| nav.push(&Page::Settings));

    html! {
        <li>
            <div onclick={settings}>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 fill-base-200 stroke-base-content stroke-[1.5px]" viewBox="0 0 24 24">
                <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>
            </svg>
                {"Settings"}
            </div>
        </li>
    }
}
