use cofd::{prelude::VariantName, schema::traits::attribute, traits::attribute::AttributeKind};
use dioxus::prelude::*;

use crate::components::dot_rating::DotRating;

#[component]
fn Attribute(name: String, value: Signal<u8>) -> Element {
	rsx! {
		span { class: "capitalize text-right sm:text-left col-span-1 text-sm font-semibold", "{name}" }
		DotRating { class: "col-span-1",
			value, max: 5 }
	}
}

#[component]
pub fn Attributes(class: String) -> Element {
	let attr = use_signal(|| 1);

	rsx! {
		div { class: "w-full grid grid-cols-2 sm:grid-cols-[repeat(7,_auto)] justify-between gap-1 text-left {class}",
			for kind in [AttributeKind::Power, AttributeKind::Finesse, AttributeKind::Resistance] {
				h3 { class: "hidden sm:inline col-span-1 text-right text-sm font-light",
					match kind {
						AttributeKind::Power => "Power",
						AttributeKind::Finesse => "Finesse",
						AttributeKind::Resistance => "Resistance",
					}
				}

				for attribute in attribute::Attribute::get_by_kind(kind) {
					Attribute { name: attribute.name(), value: attr }
				}
			}
		}
	}
}
