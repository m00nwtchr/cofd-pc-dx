use cofd::prelude::werewolf::Auspice;
use dioxus::prelude::*;

#[component]
pub fn Info(class: String) -> Element {
	let mut xsplat = use_signal(String::new);

	rsx! {
		div { class: "grid grid-cols-3 sm:grid-cols-9 gap-2 p-4 w-full {class}",
			// Row 1
			label { class: "col-span-1 text-sm font-semibold self-center", "Name:" }
			input { class: "col-span-2 border p-1 rounded", r#type: "text" }

			label { class: "col-span-1 text-sm font-semibold self-center", "Blood:" }
			input { class: "col-span-2 border p-1 rounded", r#type: "text" }

			label { class: "col-span-1 text-sm font-semibold self-center", "Auspice:" }
			select { class: "col-span-2 border p-1 rounded", onchange: move |e| xsplat.set(e.data.value()),
				for auspice in [Auspice::Rahu, Auspice::Ithaeur] {
					option {value: "{auspice:?}", "{auspice:?}" }
				}
			}

			// Row 2
			label { class: "col-span-1 text-sm font-semibold self-center", "Player:" }
			input { class: "col-span-2 border p-1 rounded", r#type: "text" }

			label { class: "col-span-1 text-sm font-semibold self-center", "Bone:" }
			input { class: "col-span-2 border p-1 rounded", r#type: "text" }

			label { class: "col-span-1 text-sm font-semibold self-center", "Tribe:" }
			select { class: "col-span-2 border p-1 rounded",
				option { value: "Bone Shadows", "Bone Shadows" }
				option { value: "Blood Talons", "Blood Talons" }
				option { value: "Hunters in Darkness", "Hunters in Darkness" }
				option { value: "Iron Masters", "Iron Masters" }
				option { value: "Storm Lords", "Storm Lords" }
			}

			// Row 3
			label { class: "col-span-1 text-sm font-semibold self-center", "Chronicle:" }
			input { class: "col-span-2 border p-1 rounded", r#type: "text" }

			label { class: "col-span-1 text-sm font-semibold self-center", "Concept:" }
			input { class: "col-span-2 border p-1 rounded", r#type: "text" }

			label { class: "col-span-1 text-sm font-semibold self-center", "Lodge:" }
			select { class: "col-span-2 border p-1 rounded",
				option { value: "", "" }
				option { value: "Example Lodge", "Example Lodge" }
			}
		}
	}
}
