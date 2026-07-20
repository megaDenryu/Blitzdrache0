//! 1プリミティブから頂点属性一覧・インデックス一覧を取り出す。

use crate::asset::error::アセットエラー;
use crate::asset::mesh_data::メッシュデータ;
use crate::asset::vertex_attribute::メッシュ頂点属性;

use super::super::document::開いた文書;

/// UV未指定のプリミティブは(0,0)充填を行う（テクスチャ無しメッシュの正常系）。
/// TANGENT未指定のプリミティブは(1,0,0,1)充填を行う（法線マップ無しマテリアルの
/// 正常系。判断25）。インデックス未指定のプリミティブは頂点順の連番を充填する
/// （非インデックス描画というglTF仕様上の正当な意味を素直に翻訳したもの）。
pub(super) fn プリミティブから取り出す(
    文書: &開いた文書,
    プリミティブ: &gltf::Primitive<'_>,
) -> Result<メッシュデータ, アセットエラー> {
    let 読み取り器 =
        プリミティブ.reader(|バッファ| 文書.バッファ一覧.get(バッファ.index()).map(Vec::as_slice));

    let 位置一覧: Vec<[f32; 3]> = 読み取り器
        .read_positions()
        .ok_or(アセットエラー::頂点位置なし)?
        .collect();
    let 頂点数 = 位置一覧.len();

    let 法線一覧: Vec<[f32; 3]> = 読み取り器
        .read_normals()
        .ok_or(アセットエラー::法線なし)?
        .collect();

    let 接線一覧: Vec<[f32; 4]> = match 読み取り器.read_tangents() {
        Some(読み取り) => 読み取り.collect(),
        None => vec![[1.0, 0.0, 0.0, 1.0]; 頂点数],
    };

    let uv一覧: Vec<[f32; 2]> = match 読み取り器.read_tex_coords(0) {
        Some(読み取り) => 読み取り.into_f32().collect(),
        None => vec![[0.0, 0.0]; 頂点数],
    };

    let インデックス一覧: Vec<u32> = match 読み取り器.read_indices() {
        Some(読み取り) => 読み取り.into_u32().collect(),
        None => {
            let 頂点数u32 = u32::try_from(頂点数)
                .map_err(|誤り| アセットエラー::解析失敗(format!("頂点数がu32の範囲を超えている: {誤り}")))?;
            (0..頂点数u32).collect()
        }
    };

    let 頂点一覧 = 位置一覧
        .into_iter()
        .zip(法線一覧)
        .zip(接線一覧)
        .zip(uv一覧)
        .map(|(((位置, 法線), 接線), uv)| メッシュ頂点属性 { 位置, 法線, 接線, uv })
        .collect();

    Ok(メッシュデータ {
        頂点一覧,
        インデックス一覧,
    })
}
