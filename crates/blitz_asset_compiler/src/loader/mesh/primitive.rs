//! 1プリミティブから頂点属性一覧・インデックス一覧を取り出す。NORMAL/TANGENTは
//! 欠如することがあるため未確定のまま返す。計算充填はメッシュ全体連結後に
//! `attribute_resolve`が行う(判断46)。

use crate::error::アセットコンパイルエラー;

use super::super::document::開いた文書;

/// NORMAL/TANGENTが未確定(欠如の可能性がある)の頂点属性。
pub(super) struct 未確定頂点属性 {
    pub(super) 位置: [f32; 3],
    pub(super) 法線: Option<[f32; 3]>,
    pub(super) 接線: Option<[f32; 4]>,
    pub(super) uv: [f32; 2],
}

/// 未確定頂点属性一覧とインデックス一覧の組。
pub(super) struct 未確定メッシュデータ {
    pub(super) 頂点一覧: Vec<未確定頂点属性>,
    pub(super) インデックス一覧: Vec<u32>,
}

/// UV未指定のプリミティブは(0,0)充填を行う（テクスチャ無しメッシュの正常系）。
/// インデックス未指定のプリミティブは頂点順の連番を充填する
/// （非インデックス描画というglTF仕様上の正当な意味を素直に翻訳したもの）。
pub(super) fn プリミティブから取り出す(
    文書: &開いた文書,
    プリミティブ: &gltf::Primitive<'_>,
) -> Result<未確定メッシュデータ, アセットコンパイルエラー> {
    let 読み取り器 = プリミティブ.reader(|バッファ| 文書.バッファ一覧.get(バッファ.index()).map(Vec::as_slice));

    let 位置一覧: Vec<[f32; 3]> = 読み取り器.read_positions().ok_or(アセットコンパイルエラー::頂点位置なし)?.collect();
    let 頂点数 = 位置一覧.len();

    let 法線一覧: Option<Vec<[f32; 3]>> = 読み取り器.read_normals().map(Iterator::collect);
    let 接線一覧: Option<Vec<[f32; 4]>> = 読み取り器.read_tangents().map(Iterator::collect);

    let uv一覧: Vec<[f32; 2]> = match 読み取り器.read_tex_coords(0) {
        Some(読み取り) => 読み取り.into_f32().collect(),
        None => vec![[0.0, 0.0]; 頂点数],
    };

    let インデックス一覧: Vec<u32> = match 読み取り器.read_indices() {
        Some(読み取り) => 読み取り.into_u32().collect(),
        None => {
            let 頂点数u32 = u32::try_from(頂点数)
                .map_err(|誤り| アセットコンパイルエラー::解析失敗(format!("頂点数がu32の範囲を超えている: {誤り}")))?;
            (0..頂点数u32).collect()
        }
    };

    let 頂点一覧 = 頂点一覧を組み立てる(位置一覧, 法線一覧, 接線一覧, uv一覧);

    Ok(未確定メッシュデータ {
        頂点一覧, インデックス一覧
    })
}

fn 頂点一覧を組み立てる(
    位置一覧: Vec<[f32; 3]>,
    法線一覧: Option<Vec<[f32; 3]>>,
    接線一覧: Option<Vec<[f32; 4]>>,
    uv一覧: Vec<[f32; 2]>,
) -> Vec<未確定頂点属性> {
    let 頂点数 = 位置一覧.len();
    let 法線オプション一覧 = 各頂点へオプション化する(法線一覧, 頂点数);
    let 接線オプション一覧 = 各頂点へオプション化する(接線一覧, 頂点数);

    位置一覧
        .into_iter()
        .zip(法線オプション一覧)
        .zip(接線オプション一覧)
        .zip(uv一覧)
        .map(|(((位置, 法線), 接線), uv)| 未確定頂点属性 { 位置, 法線, 接線, uv })
        .collect()
}

fn 各頂点へオプション化する<値>(一覧: Option<Vec<値>>, 頂点数: usize) -> Vec<Option<値>> {
    match 一覧 {
        Some(一覧) => 一覧.into_iter().map(Some).collect(),
        None => (0..頂点数).map(|_| None).collect(),
    }
}
