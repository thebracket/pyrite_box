use crate::plugins::CharacterHeader;

pub struct ChargenResource {
    pub character: CharacterHeader,
}

impl ChargenResource {
    pub fn with_character(chr: CharacterHeader) -> Self {
        Self {
            character: chr
        }
    }
}