//! 用途(usage)語彙: 画像用途・バッファ用途と、各用途→同期状態の写像。

mod buffer_usage;
pub(crate) mod buffer_usage_mapping;
mod image_usage;
pub(crate) mod image_usage_mapping;

pub(crate) use buffer_usage::バッファ用途;
pub(crate) use image_usage::画像用途;
