use cofd::{
	character::info::CharacterInfo,
	prelude::{Attribute::*, Skill::*, werewolf::*, *},
	splat::SplatCharacter,
};
use dioxus::prelude::*;

use crate::components::{Attributes, BoxRating, DotRating, Info, Merits, Skills};

#[component]
pub fn Sheet(id: usize) -> Element {
	let val = use_signal(|| 1);

	let c = use_signal(|| {
		SplatCharacter::Werewolf(
			Character::builder()
				.with_splat(
					Werewolf::new()
						.with_auspice(Auspice::Rahu)
						.with_tribe(ForsakenTribe::BloodTalons),
				)
				.with_info(CharacterInfo {
					name: String::from("Amos Gray"),
					player: String::from("m00n"),
					virtue_anchor: String::from("Destroyer"),
					vice_anchor: String::from("Lone Wolf"),
					..Default::default()
				})
				.with_attributes(vec![
					(Intelligence, 1),
					(Wits, 3),
					(Resolve, 2),
					(Strength, 3),
					(Dexterity, 2),
					(Stamina, 3),
					(Presence, 3),
					(Manipulation, 1),
					(Composure, 3),
				])
				.with_skills(vec![
					(Investigation, 2),
					(Medicine, 2),
					(Athletics, 2),
					(Brawl, 4),
					(Stealth, 2),
					(Survival, 3),
					(Expression, 3),
					(Intimidation, 4),
				])
				.build(),
		)
	});

	let character = use_context_provider(|| c);

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
							BoxRating { class: "justify-center", value: val, range: 0..=10 }
						}

						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Willpower" }
							DotRating { class: "justify-center", value: val, range: 0..=10 }
							BoxRating { class: "justify-center", value: val, range: 0..=10 }
						}

						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Primal Urge" }
							DotRating { class: "justify-center", value: val, range: 0..=10 }
						}

						div { class: "row-span-1 col-span-full",
							h3 { class: "text-md font-bold w-full", "Essence" }
							BoxRating { class: "justify-center", value: val, range: 0..=10 }
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
