//! プロパティ種別ごとのサンプラー出力読み取り。回転は正規化してf32へ揃える。

use crate::error::アセットコンパイルエラー;

pub(super) fn 平行移動出力を読む<'a, 's, F>(
    読み取り器: &gltf::animation::util::Reader<'a, 's, F>,
) -> Result<Vec<[f32; 3]>, アセットコンパイルエラー>
where
    F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
    match 読み取り器.read_outputs() {
        Some(gltf::animation::util::ReadOutputs::Translations(反復子)) => Ok(反復子.collect()),
        _ => Err(アセットコンパイルエラー::解析失敗(
            "平行移動チャンネルの出力型が不正".to_string(),
        )),
    }
}

pub(super) fn スケール出力を読む<'a, 's, F>(
    読み取り器: &gltf::animation::util::Reader<'a, 's, F>,
) -> Result<Vec<[f32; 3]>, アセットコンパイルエラー>
where
    F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
    match 読み取り器.read_outputs() {
        Some(gltf::animation::util::ReadOutputs::Scales(反復子)) => Ok(反復子.collect()),
        _ => Err(アセットコンパイルエラー::解析失敗(
            "スケールチャンネルの出力型が不正".to_string(),
        )),
    }
}

pub(super) fn 回転出力を読む<'a, 's, F>(
    読み取り器: &gltf::animation::util::Reader<'a, 's, F>,
) -> Result<Vec<[f32; 4]>, アセットコンパイルエラー>
where
    F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
    match 読み取り器.read_outputs() {
        Some(gltf::animation::util::ReadOutputs::Rotations(反復子)) => Ok(反復子.into_f32().collect()),
        _ => Err(アセットコンパイルエラー::解析失敗(
            "回転チャンネルの出力型が不正".to_string(),
        )),
    }
}
