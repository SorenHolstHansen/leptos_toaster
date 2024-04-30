use leptos::*;
use leptos_toaster::{
    Theme, Toast, ToastId, ToastOptions, ToastVariant, Toaster, ToasterPosition, Toasts,
};

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Toaster>
            <Page/>
        </Toaster>
    }
}

#[component]
fn Page() -> impl IntoView {
    let toast_context = expect_context::<Toasts>();
    let variant = RwSignal::new(ToastVariant::Normal);
    let use_rich_colors = RwSignal::new(false);
    let invert = RwSignal::new(false);
    let toast_theme = RwSignal::new(Theme::Light);
    let duration = RwSignal::new(std::time::Duration::from_secs(100));
    let position = RwSignal::new(ToasterPosition::BottomRight);

    let create_custom_toast = move |_| {
        toast_context.toast(
            view! {
                <div class="bg-gradient-to-r from-cyan-500 to-blue-500 rounded text-white p-4">"Custom toast"</div>
            },
            None,
            None,
        );
    };

    let create_toast = move |_| {
        let toast_id = ToastId::new();
        toast_context.toast(
            view! {
                <Toast
                    toast_id
                    variant=variant()
                    theme=toast_theme()
                    invert=invert()
                    rich_colors=use_rich_colors()
                    title=view! { "Title" }.into_view()
                    description=Some(view! { "Description" }.into_view())
                />
            },
            Some(toast_id),
            Some(ToastOptions {
                dismissible: true,
                duration: Some(duration()),
                position: Some(ToasterPosition::BottomLeft),
            }),
        );
    };

    view! {
        <div class="max-w-96 mx-auto pt-8">
            <h1 class="text-xl">"Leptos toaster"</h1>
            <p class="mb-8">
                "This uses the built in toast component. You can use your own component if you'd like"
            </p>

            <div class="flex flex-col">
                <label>Toast variant</label>
                <select on:change=move |ev| {
                    let new_variant = match event_target_value(&ev).as_str() {
                        "Success" => ToastVariant::Success,
                        "Info" => ToastVariant::Info,
                        "Warning" => ToastVariant::Warning,
                        "Error" => ToastVariant::Error,
                        _ => ToastVariant::Normal,
                    };
                    variant.set(new_variant);
                }>
                    <option value="Normal">Normal</option>
                    <option value="Success">Success</option>
                    <option value="Info">Info</option>
                    <option value="Warning">Warning</option>
                    <option value="Error">Error</option>
                </select>

                <label>Position</label>
                <select on:change=move |ev| {
                    let new_position = match event_target_value(&ev).as_str() {
                        "Top right" => ToasterPosition::TopRight,
                        "Top center" => ToasterPosition::TopCenter,
                        "Top left" => ToasterPosition::TopLeft,
                        "Bottom left" => ToasterPosition::BottomLeft,
                        "Bottom center" => ToasterPosition::BottomCenter,
                        _ => ToasterPosition::BottomRight,
                    };
                    position.set(new_position);
                }>
                    <option value="Bottom right">Bottom right</option>
                    <option value="Bottom center">Bottom center</option>
                    <option value="Bottom left">Bottom left</option>
                    <option value="Top left">Top left</option>
                    <option value="Top center">Top center</option>
                    <option value="Top right">Top right</option>
                </select>

                <label>Duration</label>
                <input type="number" />

                <label>Theme</label>
                <select on:change=move |ev| {
                    let new_theme = match event_target_value(&ev).as_str() {
                        "Dark" => Theme::Dark,
                        _ => Theme::Light,
                    };
                    toast_theme.set(new_theme);
                }>
                    <option value="Light">Light</option>
                    <option value="Dark">Dark</option>
                </select>

                <label>
                    <input
                        type="checkbox"
                        prop:checked=use_rich_colors
                        on:input=move |ev| use_rich_colors.set(event_target_checked(&ev))
                    />
                    Use rich colors
                    <span class="text-xs">(Can only be seen for non-normal toast variants)</span>
                </label>

                <label class="mb-4">
                    <input
                        type="checkbox"
                        prop:checked=invert
                        on:input=move |ev| invert.set(event_target_checked(&ev))
                    />
                    Invert colors
                </label>

                <button on:click=create_toast class="mt-4 rounded bg-blue-500 text-white h-10">
                    "Show toast"
                </button>

                <button on:click=create_custom_toast class="mt-4 rounded bg-blue-500 text-white h-10">
                    "Show custom toast"
                </button>
            </div>
        </div>
    }
}
