use cofd::{prelude::VariantName, schema::traits::skill};
use dioxus::prelude::*;
use strum::VariantArray;

use crate::components::dot_rating::DotRating;

#[component]
fn Skill(name: String, value: Signal<u8>) -> Element {
	rsx! {
		span { class: "capitalize text-right sm:text-left col-span-1 text-sm font-semibold", "{name}" }
		DotRating { class: "col-span-1",
			value, max: 5 }
	}
}

#[component]
pub fn Skills(class: String) -> Element {
	let attr = use_signal(|| 1);

	rsx! {
		div { class: "w-full grid grid-cols-[auto_auto] sm:grid-cols-[1fr_auto] gap-1 text-left {class}",
			for (i, attribute) in skill::Skill::VARIANTS.iter().enumerate() {
				if i % 8 == 0 {
					h3 { class: "col-span-full text-md text-center font-semibold", "Mental" }
				}
				Skill { name: attribute.name(), value: attr }
			}
		}
	}
}
