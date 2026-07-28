//! 版1内容の範囲検査付きリトルエンディアン読み書き。

mod read;
mod write;

pub(crate) use read::読取位置;
pub(crate) use write::書込先;
