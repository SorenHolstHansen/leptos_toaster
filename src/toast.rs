use leptos::*;

use crate::{types::dismiss_toast, ToastId};

#[component]
fn SuccessIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" view-box="0 0 20 20" fill="current-color" height="20" width="20">
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
              clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
fn WarningIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" view-box="0 0 24 24" fill="currentColor" height="20" width="20">
            <path
              fill-rule="evenodd"
              d="M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003zM12 8.25a.75.75 0 01.75.75v3.75a.75.75 0 01-1.5 0V9a.75.75 0 01.75-.75zm0 8.25a.75.75 0 100-1.5.75.75 0 000 1.5z"
              clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
fn InfoIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" view-box="0 0 20 20" fill="currentColor" height="20" width="20">
            <path
              fill-rule="evenodd"
              d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a.75.75 0 000 1.5h.253a.25.25 0 01.244.304l-.459 2.066A1.75 1.75 0 0010.747 15H11a.75.75 0 000-1.5h-.253a.25.25 0 01-.244-.304l.459-2.066A1.75 1.75 0 009.253 9H9z"
              clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
fn ErrorIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" view-box="0 0 20 20" fill="currentColor" height="20" width="20">
            <path
              fill-rule="evenodd"
              d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zm0 10a1 1 0 100-2 1 1 0 000 2z"
              clip-rule="evenodd"
            />
        </svg>
    }
}

pub enum ToastVariant {
    Normal,
    Success,
    Info,
    Warning,
    Error,
}

impl ToString for ToastVariant {
    fn to_string(&self) -> String {
        match self {
            ToastVariant::Normal => "normal".to_string(),
            ToastVariant::Success => "success".to_string(),
            ToastVariant::Info => "info".to_string(),
            ToastVariant::Warning => "warning".to_string(),
            ToastVariant::Error => "error".to_string(),
        }
    }
}

/// Built in toast component to use with the toast() function if you don't want to roll your own
#[component]
pub fn Toast(
    #[prop(default = ToastVariant::Normal)] variant: ToastVariant,
    title: View,
    #[prop(default = None)] description: Option<View>,
    toast_id: ToastId,
) -> impl IntoView {
    view! {
        <div style="padding: 16px; background: white; border: 1px solid hsl(0, 0%, 93%); border-radius: 8px; box-shadow: 0 4px 12px #0000001a; display: flex; align-items: center; gap: 6px; width: 356px" data-type=variant.to_string() class="leptos-toast">
            <button
                on:click=move |_| {
                    dismiss_toast(&toast_id);
                }
                data-close-button
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>

            <div class="h-full">
                {match variant {
                    ToastVariant::Normal => view! {}.into_view(),
                    ToastVariant::Success => view! {<SuccessIcon />}.into_view(),
                    ToastVariant::Info => view! {<InfoIcon />}.into_view(),
                    ToastVariant::Warning => view! {<WarningIcon />}.into_view(),
                    ToastVariant::Error => view! {<ErrorIcon />}.into_view(),
                }}
            </div>

            <div>
                <div class="leptos-toast-title">{title}</div>
                <div class="leptos-toast-description">{description}</div>
            </div>
        </div>
    }
}
