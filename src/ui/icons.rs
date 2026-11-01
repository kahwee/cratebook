//! The small subset of Phosphor Regular embedded in the gallery.
use gpui::{AssetSource, Result, SharedString};
use std::borrow::Cow;

pub(crate) struct Icons;
const FILES: &[(&str, &[u8])] = &[
    (
        "phosphor/file-text.svg",
        include_bytes!("../../assets/phosphor/file-text.svg"),
    ),
    (
        "phosphor/x.svg",
        include_bytes!("../../assets/phosphor/x.svg"),
    ),
];
impl AssetSource for Icons {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(FILES
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }
    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(FILES
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| (*name).into())
            .collect())
    }
}
