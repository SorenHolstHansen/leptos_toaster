use crate::{
    types::{decode_message, HeightT, Toast},
    ToastId, ToasterPosition,
};
use js_sys::Date;
use leptos::{ev, leptos_dom::helpers::TimeoutHandle, prelude::*};
use std::cmp::{max, min};
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, PointerEvent};

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
    let is_visible = move || index.get() < visible_toasts;
    let is_front = move || index.get() == 0;
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
            for height in heights.iter().take(height_index()) {
                acc += height.height;
            }
            acc
        })
    };
    let offset = move || (height_index() * gap) as f64 + toasts_height_before();
    let is_expanded = move || expanded.get() || (expand_by_default && mounted.get());
    let duration = toast.options.duration.unwrap_or(duration_from_toaster);
    let position = toast.options.position.unwrap_or(position);

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

    let delete_toast = move || {
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
                remove_toast.run(toast.id);
            },
            Duration::from_millis(200),
        );
    };

    // The close button calls a window.postMessage which we then pick up here and delete the toast if the ids match
    window_event_listener(ev::message, move |ev| {
        if let Some(id) = ev.data().as_string() {
            if let Some(id) = decode_message(id) {
                if id == toast.id {
                    delete_toast();
                }
            }
        }
    });

    create_effect(move |_| {
        mounted.set(true);
    });

    create_effect(move |_| {
        if let Ok(handle) = set_timeout_with_handle(delete_toast, duration) {
            delete_timeout_handle.set(Some(handle));
        }
    });

    #[derive(Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    let drag_start_time = RwSignal::<Option<Date>, LocalStorage>::new_local(None);
    let pointer_start = RwSignal::<Option<Point>>::new(None);
    let swipe_amount = RwSignal::<i32>::new(0);
    let handle_pointerdown = move |ev: PointerEvent| {
        if !toast.options.dismissible {
            return;
        }
        drag_start_time.set(Some(Date::new_0()));
        offset_before_remove.set(offset());

        if let Some(target) = ev.target() {
            if let Some(element) = target.dyn_ref::<HtmlElement>() {
                let _ = element.set_pointer_capture(ev.pointer_id());
                if element.tag_name() == "BUTTON" {
                    return;
                }
                swiping.set(true);
                pointer_start.set(Some(Point {
                    x: ev.client_x(),
                    y: ev.client_y(),
                }));
            }
        }
    };

    let handle_pointerup = move |_| {
        if swipe_out.get() || !toast.options.dismissible {
            return;
        }
        pointer_start.set(None);
        let time_taken = Date::new_0().get_time()
            - drag_start_time.with(|t| t.as_ref().map(|t| t.get_time()).unwrap_or(0.0));
        let velocity = swipe_amount.with(|a| a.abs() as f64) / time_taken;

        if swipe_amount.with(|a| a.abs() >= 20) || velocity > 0.11 {
            offset_before_remove.set(offset());
            delete_toast();
            swipe_out.set(true);
            return;
        };

        swipe_amount.set(0);
        swiping.set(false);
    };

    let handle_pointermove = move |ev: PointerEvent| {
        if !toast.options.dismissible {
            return;
        };
        let _pointer_start = if let Some(pointer_start) = pointer_start.get() {
            pointer_start
        } else {
            return;
        };

        let y_position = ev.client_y() - _pointer_start.y;
        let x_position = ev.client_x() - _pointer_start.x;

        let clamped_y = match position {
            ToasterPosition::TopLeft | ToasterPosition::TopCenter | ToasterPosition::TopRight => {
                min(0, y_position)
            }
            ToasterPosition::BottomRight
            | ToasterPosition::BottomCenter
            | ToasterPosition::BottomLeft => max(0, y_position),
        };
        let swipe_start_threshold = if ev.pointer_type() == "touch" { 10 } else { 2 };
        let is_allowed_to_swipe = clamped_y.abs() > swipe_start_threshold;

        if is_allowed_to_swipe {
            swipe_amount.set(y_position);
        } else if x_position.abs() > swipe_start_threshold {
            pointer_start.set(None);
        }
    };

    view! {
        <li
            aria-atomic="true"
            role="status"
            tab-index=0
            class="leptos-toast-container"
            data-mounted=move || mounted.get().to_string()
            data-removed=move || removed.get().to_string()
            data-visible=move || is_visible().to_string()
            data-y-position=position.y()
            data-x-position=position.x()
            data-index=index
            data-front=move || is_front().to_string()
            data-swiping=move || swiping.get().to_string()
            data-swipe-out=move || swipe_out.get().to_string()
            data-expanded=move || is_expanded().to_string()
            data-dismissible=toast.options.dismissible.to_string()
            style=("--index", move || index.get().to_string())
            style=("--toasts-before", move || index.get().to_string())
            style=("--z-index", move || (num_toasts.get() - index.get()).to_string())
            style=("--offset", move || format!("{}px", offset()))
            style=(
                "--initial-height",
                move || {
                    if expand_by_default {
                        "auto".to_string()
                    } else {
                        format!("{}px", initial_height.get())
                    }
                },
            )
            style=("--swipe-amount", move || format!("{}px", swipe_amount.get()))
            on:pointerdown=handle_pointerdown
            on:pointerup=handle_pointerup
            on:pointermove=handle_pointermove
        >
            {toast.view.run()}
        </li>
    }
}
