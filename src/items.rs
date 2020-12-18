use super::{base::Stringify, styles::FormattedText};

const KEY_VALUE_DELIM: char = ':';

pub struct MentionLink {
    text: String
}

impl MentionLink {
    pub fn new(label: &str, uid: i32) {
        let mention_link = FormattedText::hyperlink(label, &*format!("tg://user?id={}", uid));
        Self {
            text: mention_link
        };
    }
}

impl Stringify for MentionLink {
    fn stringify(&self) -> String {
        self.text.to_string()
    }
}

pub struct KeyValueItem {
    key: String,
    value: String,
}

impl KeyValueItem {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: FormattedText::bold(key),
            value: value.to_owned(),
        }
    }
}

impl Stringify for KeyValueItem {
    fn stringify(&self) -> String {
        format!("{}{} {}", self.key, KEY_VALUE_DELIM, self.value).to_string()
    }
}
