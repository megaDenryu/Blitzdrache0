//! glTF読込の入口。gltf/image型はこのモジュール配下だけに現れ、
//! asset公開APIには出さない(README「利用ライブラリ」封じ込め規約)。

mod animation;
mod buffer;
mod document;
mod file;
mod material;
mod mesh;
mod skin;
mod texture_decode;

use crate::asset::catalog::カタログ;
use crate::asset::chunk_id::チャンクID;
use crate::asset::error::アセットエラー;
use crate::asset::id::アセットID;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::render_object_id::描画対象ID;
use crate::asset::scene_data::シーンデータ;

const 単一アセット描画対象番号: u64 = 0;
const 単一アセット所有チャンク番号: u64 = 0;

/// カタログでidを引き、glTF(.gltf/.glb)の最初のメッシュを読み込む。
/// メッシュ内の全プリミティブを頂点オフセット付きで連結し(判断37)、
/// マテリアルは先頭プリミティブのものをメッシュ全体へ適用する前提を維持する。
/// 複数メッシュ・階層はM3スコープ外(参照: 開発スレッド「判断22」)。
/// スキンを持つノードに紐づくメッシュなら、スキンとアニメーション一覧も読み込む(判断42)。
pub fn シーンを読み込む(カタログ: &カタログ, id: &アセットID) -> Result<シーンデータ, アセットエラー> {
    let パス = カタログ.パスを引く(id).ok_or_else(|| アセットエラー::カタログ未登録(id.clone()))?;

    let 開いた文書 = document::文書を開く(パス)?;
    let 対象メッシュ = 開いた文書.document.meshes().next().ok_or(アセットエラー::メッシュなし)?;
    let 先頭プリミティブ = 対象メッシュ.primitives().next().ok_or(アセットエラー::プリミティブなし)?;

    let スキン読込結果 = skin::スキンを取り出す(&開いた文書, 対象メッシュ.index())?;

    let メッシュ実体 = mesh::メッシュデータを取り出す(&開いた文書, &対象メッシュ, スキン読込結果.as_ref().map(|(_, 関節解決)| 関節解決))?;
    let (マテリアル, マテリアル参照ファイル一覧) = material::マテリアルを取り出す(&開いた文書, &先頭プリミティブ)?;

    let アニメーション一覧 = match &スキン読込結果 {
        Some((データ, 関節解決)) => animation::アニメーション一覧を取り出す(&開いた文書, 関節解決, データ.ジョイント一覧.len())?,
        None => Vec::new(),
    };

    let mut 参照ファイル一覧 = 開いた文書.参照ファイル一覧;
    参照ファイル一覧.extend(マテリアル参照ファイル一覧);

    let ローカルからワールド =
        blitz_math::変換::列優先配列から生成する(
            [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
        );
    let 描画対象 = 描画対象データ::生成する(
        描画対象ID::生成する(単一アセット描画対象番号),
        チャンクID::生成する(単一アセット所有チャンク番号),
        ローカルからワールド,
        メッシュ実体,
        マテリアル,
    );

    Ok(シーンデータ::生成する(
        描画対象,
        Vec::new(),
        参照ファイル一覧,
        スキン読込結果.map(|(データ, _)| データ),
        アニメーション一覧,
    ))
}
