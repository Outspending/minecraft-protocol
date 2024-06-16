use simdnbt::owned::{Nbt, NbtCompound, NbtTag};

pub struct ChatType<'a> {
    pub name: &'a str,
    pub chat: ChatDecoration<'a>,
    pub narrator: ChatDecoration<'a>
}

impl<'a> ChatType<'a> {
    pub fn new(name: &'a str, chat: ChatDecoration<'a>, narrator: ChatDecoration<'a>) -> Self {
        Self {
            name,
            chat,
            narrator
        }
    }
    
    pub fn to_nbt(&self) -> Nbt {
        Nbt::new(
            "".into(),
            NbtCompound::from_values(vec![
                ("chat".into(), self.chat.to_nbt()),
                ("narration".into(), self.narrator.to_nbt())
            ]),
        )
    }
}

pub struct ChatDecoration<'a> {
    pub name: &'a str,
    pub translation_key: &'a str,
    pub parameters: Vec<String>,
}

impl<'a> ChatDecoration<'a> {
    pub fn new(name: &'a str, translation_key: &'a str, parameters: Vec<&'a str>) -> Self {
        Self {
            name,
            translation_key,
            parameters: parameters.iter().map(|&x| x.to_string()).collect::<Vec<String>>(),
        }
    }

    pub fn to_nbt(&self) -> NbtTag {
        NbtTag::Compound(
            NbtCompound::from_values(vec![
                (
                    "name".into(),
                    NbtTag::String(self.name.into()),
                ),
                (
                    "translation_key".into(),
                    NbtTag::String(self.translation_key.into()),
                ),
                (
                    "parameters".into(),
                    NbtTag::List(self.parameters.clone().into()),
                ),
            ]),
        )
    }
}