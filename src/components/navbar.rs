use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
	rsx! {
		nav {
			id: "navbar",
			Link {
				to: Route::Sheet { id: 0 },
				"Sheet"
			}
		}

		Outlet::<Route> {}
	}
}
