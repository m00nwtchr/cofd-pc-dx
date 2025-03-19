use dioxus::prelude::*;

use crate::components::{Attributes, Info};

#[component]
pub fn Sheet(id: usize) -> Element {
	rsx! {
		main {
			class: "mx-auto w-6xl",
			Info {}
			Attributes {}
		}
	}
}
