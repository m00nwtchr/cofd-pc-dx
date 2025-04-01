use cofd::prelude::VariantName;
use dioxus::prelude::*;

use crate::components::dot_rating::DotRating;

#[component]
fn Merit(name: String, value: Signal<u8>) -> Element {
	rsx! {
		select { class: "col-span-1 border-b px-1 h-6",
			option {value: "D", "D"}
		}
		DotRating { class: "col-span-1",
			value, max: 5 }
	}
}

#[component]
pub fn Merits(class: String) -> Element {
	let attr = use_signal(|| 1);

	rsx! {
		div { class: "w-full grid grid-cols-[1fr_auto] gap-1 text-left {class}",
			for _ in 0..12 {
				Merit { name: "", value: attr }
			}
		}
	}
}
