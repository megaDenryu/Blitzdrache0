//! glTF読込の入口。gltf/image型はこのモジュール配下だけに現れ、
//! asset公開APIには出さない(README「利用ライブラリ」封じ込め規約)。

mod buffer;
mod document;
mod file;
mod mesh;
mod texture;

use crate::asset::catalog::カタログ;
use crate::asset::error::アセットエラー;
use crate::asset::id::アセットID;
use crate::asset::scene_data::シーンデータ;

/// カタログでidを引き、glTF(.gltf/.glb)の最初のメッシュ・最初のプリミティブを
/// 読み込む。複数メッシュ・階層はM3スコープ外(参照: 開発スレッド「判断22」)。
pub fn シーンを読み込む(カタログ: &カタログ, id: &アセットID) -> Result<シーンデータ, アセットエラー> {
    let パス = カタログ
        .パスを引く(id)
        .ok_or_else(|| アセットエラー::カタログ未登録(id.clone()))?;

    let 開いた文書 = document::文書を開く(パス)?;
    let 対象メッシュ = 開いた文書
        .document
        .meshes()
        .next()
        .ok_or(アセットエラー::メッシュなし)?;
    let プリミティブ = 対象メッシュ
        .primitives()
        .next()
        .ok_or(アセットエラー::プリミティブなし)?;

    let メッシュ実体 = mesh::メッシュデータを取り出す(&開いた文書, &プリミティブ)?;
    let (ベースカラー, テクスチャ参照パス) = texture::ベースカラーを取り出す(&開いた文書, &プリミティブ)?;

    let mut 参照ファイル一覧 = 開いた文書.参照ファイル一覧;
    if let Some(パス) = テクスチャ参照パス {
        参照ファイル一覧.push(パス);
    }

    Ok(シーンデータ {
        メッシュ: メッシュ実体,
        ベースカラー,
        参照ファイル一覧,
    })
}
