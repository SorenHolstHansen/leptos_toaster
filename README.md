![Example](assets/example.gif)

# leptos_toaster

A Toaster component for Leptos heavily inspired by [sonner](https://sonner.emilkowal.ski/)

## Usage
Somewhere, probably near the top of your component tree, add the Toaster component
```rust
view! {
	<Toaster
	    position=toaster::ToasterPosition::BottomCenter
	>
		// ...
	</Toaster>
}
```

and then whenever you need a toast, do

```rust
let toast_context = expect_context::<Toasts>();

let create_toast = move || {
	let toast_id = ToastId::new();
	toast_context.toast(
		// This uses the built in toast component that requires the `builtin_toast` feature.
		// You can use your own components here
		view! { 
			<Toast 
				toast_id 
				variant=ToastVariant::Info 
				title=view! {"My toast"}.into_view() 
			/> 
		},
		Some(toast_id),
		None // options
	);
}
```
