use cofd::{prelude::VariantName, schema::traits::attribute};
use dioxus::prelude::*;
use strum::VariantArray;

use crate::components::dot_rating::DotRating;

#[component]
fn Attribute(name: String, value: Signal<u8>) -> Element {
	rsx! {
		span { class: "capitalize col-span-1 text-sm font-semibold", "{name}" }
		DotRating { class: "col-span-2",
			value, max: 5 }
	}
}

#[component]
pub fn Attributes() -> Element {
	let attr = use_signal(|| 1);

	rsx! {
		div { class: "p-4 w-full text-center",
			h2 { class: "col-span-3 text-lg font-bold", "Attributes" }
			div { class: "w-full grid grid-cols-10 gap-1 text-left",
				for (i, attribute) in attribute::Attribute::VARIANTS.iter().enumerate() {
					if i % 3 == 0 {
						span { class: "col-span-1 text-right text-sm font-light",
							match i {
								0 => "Power",
								3 => "Finesse",
								6 => "Resistance",
								_ => ""
							}
						}
					}
					Attribute { name: attribute.name(), value: attr }
				}
			}
		}
	}
}
