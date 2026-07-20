//! 用途(usage)語彙: 画像用途・バッファ用途と、画像用途→同期状態の写像。

mod buffer_usage;
mod image_usage;
pub(crate) mod image_usage_mapping;

#[allow(unused_imports)]
pub(crate) use buffer_usage::バッファ用途;
pub(crate) use image_usage::画像用途;
