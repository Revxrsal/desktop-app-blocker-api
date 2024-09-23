use crate::blocker::text::TextPredicate;

#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
pub enum AppBlockAction {
    Close,
    MinimizeWindow,
}
