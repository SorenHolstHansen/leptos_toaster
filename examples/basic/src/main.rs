use leptos::*;
use leptos_toaster::{Toast, ToastId, ToastVariant, Toaster, ToasterPosition, Toasts};

fn main() {
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Toaster
            position=ToasterPosition::BottomCenter
        >
            <h1>"Basic example"</h1>

            <Page />
        </Toaster>
    }
}

#[component]
fn Page() -> impl IntoView {
    let toast_context = expect_context::<Toasts>();

    let create_toast = move |_| {
        let toast_id = ToastId::new();
        toast_context.toast(
            view! {
                <Toast
                    toast_id
                    variant=ToastVariant::Info
                    title=view! { "My toast" }.into_view()
                />
            },
            Some(toast_id),
            None,
        );
    };

    view! {
        <button on:click=create_toast>"Add toast"</button>
    }
}
