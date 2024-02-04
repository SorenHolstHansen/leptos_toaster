use crate::{
    toast_container::ToastContainer,
    types::{HeightT, Toasts},
    ToastId, ToasterPosition,
};
use leptos::*;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, PointerEvent};

/// Toaster context provider.
/// Wrap your app in the Toaster to use the Toasts context in children
#[component]
pub fn Toaster(
    #[prop(default = ToasterPosition::BottomRight)] position: ToasterPosition,
    #[prop(default = false)] expand: bool,
    #[prop(default = Duration::from_millis(4000))] duration: Duration,
    #[prop(default = 14)] gap: usize,
    /// The maximum amount of toasts that should be visible at any point
    #[prop(default = 3)]
    visible_toasts: usize,
    children: Children,
) -> impl IntoView {
    let (toasts, set_toasts) = create_signal(Vec::new());
    let (expanded, set_expanded) = create_signal(false);
    let interacting = RwSignal::new(false);
    let heights = RwSignal::<Vec<HeightT>>::new(Vec::new());

    provide_context(Toasts { set_toasts });
    create_effect(move |_| {
        // Ensure expanded is always false when no toasts are present / only one left
        if toasts.with(|t| t.len() <= 1) {
            set_expanded(false);
        }
    });

    let remove_toast = Callback::new(move |toast_id: ToastId| {
        set_toasts.update(|toasts| {
            if let Some(index) = toasts.iter().position(|t| t.id == toast_id) {
                toasts.remove(index);
            }
        });
    });

    let on_pointerdown = move |e: PointerEvent| {
        let mut is_dismissible = true;
        if let Some(target) = e.target() {
            if let Some(element) = target.dyn_ref::<HtmlElement>() {
                if let Some(dismissible) = element.dataset().get("dismissible") {
                    is_dismissible = dismissible != "false";
                }
            };
        };
        if is_dismissible {
            interacting.set(true);
        }
    };

    view! {
        {children()}

        <Show
            when=move || !toasts.with(|t| t.is_empty())
        >
        <section
            aria-label="Notifications"
            tab-index={-1}
        >
            <ol
                class="leptos-toaster"
                tab-index={-1}
                data-y-position=position.y()
                data-x-position=position.x()
                style=("--gap", format!("{}px", gap))
                style=("--width", "356px")
                style=("--offset", "32px")
                style=("--front-toast-height", move || format!("{}px", heights.with(|heights| heights.get(0).map(|h| h.height).unwrap_or(0.0))))
                on:mouseenter=move |_| set_expanded(true)
                on:mousemove=move |_| set_expanded(true)
                on:mouseleave=move |_| if !interacting() { set_expanded(false) }
                on:pointerdown=on_pointerdown
                on:pointerup=move |_| interacting.set(true)
            >
                <For
                    each=move || toasts()
                    key=move |toast| toast.id
                    children=move |toast| {
                        // Doing this since we
                        // 1. don't want the view to rerender, and in turn, the ToastContainer to rerender when a new toast is added, because that makes the internal logic more complex. For instance the timeout to delete the toast after the duration would have to keep track of the timeout handle between rerenders. And
                        // 2. enumerating the toasts vec will not give a reactive index, so we need to get it like this
                        let index = create_memo(move |_| {
                            toasts.with(|toasts| toasts.iter().position(|t| t.id == toast.id).unwrap_or_default())
                        });
                        view! {
                            <ToastContainer
                                index=Signal::derive(move || index.get())
                                toast
                                visible_toasts
                                position
                                duration_from_toaster=duration
                                remove_toast=remove_toast
                                expanded
                                expand_by_default=expand
                                num_toasts=Signal::derive(move || toasts.with(|t| t.len()))
                                heights
                                gap
                            />
                        }
                    }
                />
            </ol>
        </section>
        </Show>
    }
}
