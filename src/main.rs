#![warn(clippy::pedantic)]

use dioxus::prelude::*;

mod components;
mod router;
mod views;

use router::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
	LaunchBuilder::new()
		.with_cfg(desktop! {
			#[cfg(feature = "desktop")]
			{
				use dioxus::desktop::{Config, WindowBuilder};
				Config::default().with_window(WindowBuilder::default().with_decorations(false))
			}
		})
		.with_cfg(server_only! {
			ServeConfig::builder()
				// Enable incremental rendering
				.incremental(
					IncrementalRendererConfig::new()
						// Store static files in the public directory where other static assets like wasm are stored
						.static_dir(
							std::env::current_exe()
								.unwrap()
								.parent()
								.unwrap()
								.join("public")
						)
						// Don't clear the public folder on every build. The public folder has other files including the wasm
						// binary and static assets required for the app to run
						.clear_cache(false)
				)
				.enable_out_of_order_streaming()
		})
		.launch(App);
}

#[component]
fn App() -> Element {
	// Build cool things ✌️

	rsx! {
		// Global app resources
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }

		document::Link { rel: "preconnect", href: "https://rsms.me/" }
		document::Link { rel: "stylesheet", href: "https://rsms.me/inter/inter.css" }

		Router::<Route> {}
	}
}
