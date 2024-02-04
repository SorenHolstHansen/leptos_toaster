use crate::{
    types::{decode_message, HeightT, Toast},
    ToastId, ToasterPosition,
};
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use std::time::Duration;

#[component]
pub fn ToastContainer(
    index: Signal<usize>,
    toast: Toast,
    duration_from_toaster: Duration,
    visible_toasts: usize,
    position: ToasterPosition,
    #[prop(into)] remove_toast: Callback<ToastId>,
    expanded: ReadSignal<bool>,
    expand_by_default: bool,
    num_toasts: Signal<usize>,
    heights: RwSignal<Vec<HeightT>>,
    gap: usize,
) -> impl IntoView {
    let mounted = RwSignal::new(false);
    let removed = RwSignal::new(false);
    let swiping = RwSignal::new(false);
    let swipe_out = RwSignal::new(false);
    let is_visible = move || index() + 1 <= visible_toasts;
    let is_front = move || index() == 0;
    let height_index = move || {
        heights.with(|heights| {
            heights
                .iter()
                .position(|height| height.toast_id == toast.id)
                .unwrap_or(0)
        })
    };
    let toasts_height_before = move || {
        heights.with(|heights| {
            let mut acc = 0.0;
            for reducer_index in 0..height_index() {
                acc += heights[reducer_index].height;
            }
            acc
        })
    };
    let offset = move || (height_index() * gap) as f64 + toasts_height_before();
    let is_expanded = move || {
        return expanded() || (expand_by_default && mounted());
    };
    let duration = toast.options.duration.unwrap_or(duration_from_toaster);

    let initial_height = RwSignal::new(0.0);
    let offset_before_remove = RwSignal::new(0.0);

    create_effect(move |_| {
        if let Some(document) = window().document() {
            if let Ok(Some(toast_container_node)) =
                document.query_selector(".leptos-toast-container")
            {
                let height = toast_container_node.get_bounding_client_rect().height();
                initial_height.set(height);
                heights.update(|heights| {
                    heights.insert(
                        0,
                        HeightT {
                            toast_id: toast.id,
                            height,
                        },
                    )
                });
            }
        }
    });

    let delete_timeout_handle = RwSignal::<Option<TimeoutHandle>>::new(None);

    let delete_toast = move |_| {
        removed.set(true);
        offset_before_remove.set(offset());
        heights.update(|heights| {
            if let Some(i) = heights.iter().position(|t| t.toast_id == toast.id) {
                heights.remove(i);
            }
        });

        set_timeout(
            move || {
                // If the toast was deleted by the close button, we stop the timeout that would otherwise delete the toast a time im the future when it already has been disposed
                if let Some(handle) = delete_timeout_handle.get() {
                    handle.clear();
                }
                remove_toast(toast.id);
            },
            Duration::from_millis(200),
        );
    };

    // The close button calls a window.postMessage which we then pick up here and delete the toast if the ids match
    window_event_listener(ev::message, move |ev| {
        if let Some(id) = ev.data().as_string() {
            if let Some(id) = decode_message(id) {
                if id == toast.id {
                    delete_toast(id);
                }
            }
        }
    });

    create_effect(move |_| {
        mounted.set(true);
    });

    create_effect(move |_| {
        if let Ok(handle) = set_timeout_with_handle(move || delete_toast(toast.id), duration) {
            delete_timeout_handle.set(Some(handle));
        }
    });

    view! {
        <li
            aria-atomic="true"
            role="status"
            tab-index=0
            class="leptos-toast-container"
            data-mounted=move || mounted().to_string()
            data-removed=move || removed().to_string()
            data-visible=move || is_visible().to_string()
            data-y-position=position.y()
            data-x-position=position.x()
            data-index=move || index()
            data-front=move || is_front().to_string()
            data-swiping=move || swiping().to_string()
            data-swipe-out=move || swipe_out().to_string()
            data-expanded=move || is_expanded().to_string()
            data-dismissible=toast.options.dismissible.to_string()
            style=("--index", move || index())
            style=("--toasts-before", move || index())
            style=("--z-index", move || num_toasts() - index())
            style=("--offset", move || format!("{}px", offset()))
            style=("--initial-height", move || if expand_by_default {"auto".to_string()} else { format!("{}px", initial_height()) })
        >
            {toast.view}
        </li>
    }
}
