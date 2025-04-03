use cofd::{prelude::VariantName, schema::traits::skill};
use dioxus::prelude::*;
use strum::VariantArray;

use crate::{
	components::dot_rating::DotRating,
	hooks::{set_skill, use_character, use_skill},
};

#[component]
fn Skill(name: String, value: ReadOnlySignal<u8>, onchange: Option<EventHandler<u8>>) -> Element {
	rsx! {
		span { class: "capitalize text-right sm:text-left col-span-1 text-sm font-semibold", "{name}" }
		DotRating { class: "col-span-1",
			value, range: 0..=5, onchange }
	}
}

#[component]
pub fn Skills(class: String) -> Element {
	let character = use_character();

	rsx! {
		div { class: "w-full grid grid-cols-[auto_auto] sm:grid-cols-[1fr_auto] gap-1 text-left {class}",
			for (i, skill) in skill::Skill::VARIANTS.iter().enumerate() {
				if i % 8 == 0 {
					h3 { class: "col-span-full text-md text-center font-semibold", "Mental" }
				}
				Skill { name: skill.name(),
					value: use_skill(character, *skill),
					onchange: move |v| set_skill(character, *skill, v)
				}
			}
		}
	}
}
