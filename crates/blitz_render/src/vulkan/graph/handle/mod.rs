//! 仮想リソースのハンドル型2種（画像・バッファ）。

mod buffer_handle;
mod image_handle;

#[allow(unused_imports)]
pub(crate) use buffer_handle::バッファハンドル;
pub(crate) use image_handle::画像ハンドル;
