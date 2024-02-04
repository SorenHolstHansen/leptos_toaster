#![deny(unused_crate_dependencies)]

mod mount_style;
#[cfg(feature = "builtin_toast")]
mod toast;
mod toast_container;
mod toast_id;
mod toaster;
mod types;

#[cfg(feature = "builtin_toast")]
pub use toast::{Toast, ToastVariant};
pub use toast_id::ToastId;
pub use toaster::Toaster;
pub use types::{dismiss_toast, ToastOptions, ToasterPosition, Toasts};
