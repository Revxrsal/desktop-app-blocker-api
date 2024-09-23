use std::ops::{BitAnd, BitOr, Not};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Clone, Debug)]
pub enum TextPredicate {
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    Exact(String),
    Not(Box<TextPredicate>),
    And(Box<TextPredicate>, Box<TextPredicate>),
    Or(Box<TextPredicate>, Box<TextPredicate>),
}

impl TextPredicate {
    pub fn exact<S: AsRef<str>>(text: S) -> TextPredicate {
        TextPredicate::Exact(text.as_ref().to_owned())
    }

    pub fn starts_with<S: AsRef<str>>(text: S) -> TextPredicate {
        TextPredicate::StartsWith(text.as_ref().to_owned())
    }

    pub fn ends_with<S: AsRef<str>>(text: S) -> TextPredicate {
        TextPredicate::EndsWith(text.as_ref().to_owned())
    }

    pub fn contains<S: AsRef<str>>(text: S) -> TextPredicate {
        TextPredicate::Contains(text.as_ref().to_owned())
    }

    pub fn test(&self, text: &str) -> bool {
        let text = text.to_lowercase();
        match self {
            TextPredicate::StartsWith(x) => text.starts_with(&x.to_lowercase()),
            TextPredicate::EndsWith(x) => text.ends_with(&x.to_lowercase()),
            TextPredicate::Contains(x) => text.contains(&x.to_lowercase()),
            TextPredicate::Exact(x) => text.eq_ignore_ascii_case(x),
            TextPredicate::Not(x) => !x.test(&text),
            TextPredicate::And(a, b) => a.test(&text) && b.test(&text),
            TextPredicate::Or(a, b) => a.test(&text) || b.test(&text),
        }
    }
}

impl BitAnd for TextPredicate {
    type Output = TextPredicate;

    fn bitand(self, rhs: Self) -> Self::Output {
        TextPredicate::And(Box::from(self), Box::from(rhs))
    }
}

impl BitOr for TextPredicate {
    type Output = TextPredicate;

    fn bitor(self, rhs: Self) -> Self::Output {
        TextPredicate::Or(Box::from(self), Box::from(rhs))
    }
}

impl Not for TextPredicate {
    type Output = TextPredicate;

    fn not(self) -> Self::Output {
        TextPredicate::Not(Box::from(self))
    }
}
