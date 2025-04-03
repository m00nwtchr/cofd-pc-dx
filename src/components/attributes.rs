use cofd::{
	prelude::{self, *},
	splat::SplatCharacter,
};
use dioxus::prelude::*;

use crate::{
	components::dot_rating::DotRating,
	hooks::{set_attr, use_attr, use_character},
};

#[component]
fn AttributeComponent(
	name: String,
	value: Memo<u8>,
	onchange: Option<EventHandler<u8>>,
) -> Element {
	rsx! {
		span { class: "capitalize text-right sm:text-left col-span-1 text-sm font-semibold", "{name}" }
		DotRating { class: "col-span-1",
			value, range: 1..=5, onchange }
	}
}

#[component]
pub fn Attributes(class: String) -> Element {
	let character = use_character();

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

				for attribute in prelude::Attribute::get_by_kind(kind) {
					AttributeComponent {
						name: attribute.name(),
						value: use_attr(character, attribute),
						onchange: move |v| set_attr(character, attribute, v),
					}
				}
			}
		}
	}
}
