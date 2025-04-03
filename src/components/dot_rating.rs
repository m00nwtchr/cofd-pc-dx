use std::ops::RangeInclusive;

use dioxus::prelude::*;

#[component]
pub fn BoxRating(
	class: Option<String>,
	bclass: Option<String>,
	value: ReadOnlySignal<u8>,
	range: RangeInclusive<u8>,
	onchange: Option<EventHandler<u8>>,
) -> Element {
	let bclass = bclass.unwrap_or_default() + " peer h-5 w-5 aspect-square border cursor-pointer";
	let class = class.unwrap_or_default();

	rsx! {
		div { class: "flex items-center gap-1 group {class}",
			for i in 0..(*range.end()) {
				div {
					class: if i < value() {
						"{bclass} border-accent bg-accent"
					} else {
						"{bclass} peer-hover:bg-white/0! border-black group-has-[div:hover]:bg-accent/30 hover:bg-accent/30"
					},
					onclick: {
						let range = range.clone();
						move |_| if let Some(onchange) = onchange {
							let value =	if value() == i + 1 { i } else { i + 1 };

							onchange.call(value.clamp(*range.start(), *range.end()));
						}
					},
				}
			}
		}
	}
}

#[component]
pub fn DotRating(
	class: Option<String>,
	value: ReadOnlySignal<u8>,
	range: RangeInclusive<u8>,
	onchange: Option<EventHandler<u8>>,
) -> Element {
	rsx! {
		BoxRating { class, bclass: "rounded-full", value, range, onchange }
	}
}
