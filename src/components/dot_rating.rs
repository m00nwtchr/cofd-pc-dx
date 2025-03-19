use dioxus::prelude::*;

#[component]
pub fn DotRating(class: String, value: Signal<u8>, max: u8) -> Element {
	rsx! {
		div { class: "flex gap-1 group {class}",
			for i in 0..max {
				div {
					class: if i < value() {
						"w-4 h-4 border rounded-full border-blue-600 bg-blue-600"
					} else {
						"peer peer-hover:bg-white w-4 h-4 border rounded-full border-blue-600 group-hover:bg-blue-200 hover:bg-blue-200"
					},
					onclick: move |_| value.set(i+1),
				}
			}
		}
	}
}
