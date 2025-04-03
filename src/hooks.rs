use cofd::{
	prelude::{self, *},
	splat::SplatCharacter,
};
use dioxus::{
	hooks::use_memo,
	prelude::consume_context,
	signals::{Memo, Signal, Writable},
};

pub fn use_character() -> Signal<SplatCharacter> {
	consume_context()
}

pub fn use_attr(char: Signal<SplatCharacter>, attr: prelude::Attribute) -> Memo<u8> {
	use_memo(move || {
		char()
			.attributes()
			.value(&Trait::Attribute(attr))
			.unwrap_or_default()
	})
}

pub fn set_attr(mut char: Signal<SplatCharacter>, attr: prelude::Attribute, value: u8) {
	char.with_mut(|ca| {
		ca.attributes_mut()
			.set_raw_value(&Trait::Attribute(attr), value);
	});
}

pub fn use_skill(char: Signal<SplatCharacter>, skill: prelude::Skill) -> Memo<u8> {
	use_memo(move || {
		char()
			.attributes()
			.value(&Trait::Skill(skill))
			.unwrap_or_default()
	})
}

pub fn set_skill(mut char: Signal<SplatCharacter>, skill: prelude::Skill, value: u8) {
	char.with_mut(|ca| {
		ca.attributes_mut()
			.set_raw_value(&Trait::Skill(skill), value);
	});
}
