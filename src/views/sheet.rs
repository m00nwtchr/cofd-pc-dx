use dioxus::prelude::*;

use crate::components::{Attributes, BoxRating, DotRating, Info, Merits, Skills};

#[component]
pub fn Sheet(id: usize) -> Element {
	let val = use_signal(|| 1);

	rsx! {
		main {
			class: "mx-auto sm:w-6xl grid grid-cols-1 sm:grid-cols-[repeat(auto-fit,_minmax(200px,_1fr))]",
			Info { class: "col-span-full" }

			div {
				class: "grid grid-cols-subgrid p-4 col-span-full",
				h2 { class: "col-span-3 text-lg text-center col-span-full font-bold", "Attributes" }
				Attributes { class: "col-span-full" }
			}

			div {
				class: "grid grid-cols-1 grid-rows-subgrid p-4 text-center col-span-2 row-span-2",
				h2 { class: "text-lg font-bold col-span-full row-span-1", "Skills" }
				Skills { class: "col-span-full row-span-1" }
			}

			div { class: "grid grid-cols-subgrid grid-rows-subgrid p-4 text-center col-span-1 sm:col-start-[3] sm:col-end-[-1] row-span-2",
				h2 { class: "text-lg font-bold col-span-full", "Other Traits" }
				div { class: "col-span-full grid grid-cols-1 sm:grid-cols-2",
					div { class: "col-span-1 row-span-8 grid grid-cols-1 grid-rows-subgrid",
						div { class: "row-span-4 col-span-full",
							h3 { class: "text-md font-bold w-full", "Merits" }
							Merits { class: "" }
						}
						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Renown" }
							for _ in 0..4 {
								div { "A" }
							}
						}

						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Aspirations" }
							for _ in 0..4 {
								input { class: "w-full border-b p-1", r#type: "text" }
							}
						}
						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Conditions" }
							for _ in 0..4 {
								input { class: "w-full border-b p-1", r#type: "text" }
							}
						}
					}

					div { class: "col-span-1 row-span-8 grid grid-cols-1 grid-rows-subgrid",
						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Health" }
							BoxRating { class: "justify-center", value: val, max: 10 }
						}

						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Willpower" }
							DotRating { class: "justify-center", value: val, max: 10 }
							BoxRating { class: "justify-center", value: val, max: 10 }
						}

						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Primal Urge" }
							DotRating { class: "justify-center", value: val, max: 10 }
						}

						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Essence" }
							BoxRating { class: "justify-center", value: val, max: 10 }
						}

						div { class: "row-span-4 col-span-full grid grid-cols-1",
							h3 { class: "text-md font-bold col-span-full row-span-1", "Flesh Touchstone" }
							h3 { class: "text-md font-bold col-span-full row-span-1", "Harmony" }
							h3 { class: "text-md font-bold col-span-full row-span-1", "Spirit Touchstone" }

							h3 { class: "text-md font-bold col-span-full row-span-2", "Kuruth Triggers" }
						}
					}
				}
			}
		}
	}
}
