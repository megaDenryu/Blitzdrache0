//! メッシュ内の全プリミティブを頂点オフセット付きで連結する(判断37)。
//! 個々のプリミティブからの抽出は`primitive`に委ねる。スキンを持つメッシュは
//! ジョイント/ウェイトも同じ頂点順で連結する(判断42)。

mod attribute_resolve;
mod normal_fill;
#[cfg(test)]
mod normal_fill_tests;
mod primitive;
mod skin_vertex;
mod tangent_fill;
#[cfg(test)]
mod tangent_fill_tests;
mod vector_math;

use crate::asset::error::アセットエラー;
use crate::asset::mesh_data::メッシュデータ;

use super::document::開いた文書;
use super::skin::関節解決;

/// 前提: マテリアルは呼び出し元(loader::mod)が先頭プリミティブのものだけを読み、
/// メッシュ全体へ適用する(同一マテリアル前提。参照: 開発スレッド「判断37」)。
/// ここでは全プリミティブの頂点・インデックスを、後続プリミティブのインデックスへ
/// 先行プリミティブの頂点数ぶんのオフセットを加算しながら連結する。
/// `関節解決`が`Some`のときのみスキン頂点属性も併せて連結する。
/// NORMAL/TANGENT欠如の計算充填(判断46)は連結後の全体に対して1回だけ行う。
pub(super) fn メッシュデータを取り出す(
    文書: &開いた文書,
    メッシュ: &gltf::Mesh<'_>,
    関節解決: Option<&関節解決>,
) -> Result<メッシュデータ, アセットエラー> {
    let mut 未確定頂点一覧 = Vec::new();
    let mut インデックス一覧 = Vec::new();
    let mut スキン頂点属性一覧 = 関節解決.map(|_| Vec::new());

    for プリミティブ in メッシュ.primitives() {
        let プリミティブデータ = primitive::プリミティブから取り出す(文書, &プリミティブ)?;
        let 頂点オフセット = u32::try_from(未確定頂点一覧.len()).map_err(|誤り| {
            アセットエラー::解析失敗(format!("頂点オフセットがu32の範囲を超えている: {誤り}"))
        })?;
        インデックス一覧.extend(
            プリミティブデータ
                .インデックス一覧
                .into_iter()
                .map(|添字| 添字 + 頂点オフセット),
        );

        if let (Some(関節解決), Some(格納先)) = (関節解決, スキン頂点属性一覧.as_mut()) {
            let 属性一覧 = skin_vertex::プリミティブのスキン属性を取り出す(
                文書,
                &プリミティブ,
                関節解決,
                プリミティブデータ.頂点一覧.len(),
            )?;
            格納先.extend(属性一覧);
        }

        未確定頂点一覧.extend(プリミティブデータ.頂点一覧);
    }

    let 頂点一覧 = attribute_resolve::頂点属性を確定する(未確定頂点一覧, &インデックス一覧);

    Ok(メッシュデータ {
        頂点一覧,
        インデックス一覧,
        スキン頂点属性一覧,
    })
}
