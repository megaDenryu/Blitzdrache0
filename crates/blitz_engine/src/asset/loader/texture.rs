//! マテリアルのbaseColorテクスチャをRGBA8にデコードして取り出す。

use crate::asset::error::アセットエラー;
use crate::asset::texture_data::テクスチャデータ;

use super::document::開いた文書;
use super::file::外部ファイルを読む;

/// baseColorテクスチャが無いプリミティブは`None`を返す（正常系。無言の
/// デフォルト適用はせず、白テクスチャへの補完は呼び出し側の判断に委ねる）。
pub(super) fn ベースカラーを取り出す(
    文書: &開いた文書,
    プリミティブ: &gltf::Primitive<'_>,
) -> Result<Option<テクスチャデータ>, アセットエラー> {
    let 情報 = match プリミティブ
        .material()
        .pbr_metallic_roughness()
        .base_color_texture()
    {
        Some(情報) => 情報,
        None => return Ok(None),
    };

    let 画像 = 情報.texture().source();
    let バイト列 = 画像バイト列を取り出す(文書, &画像)?;

    let デコード画像 = image::load_from_memory(&バイト列)
        .map_err(|誤り| アセットエラー::画像デコード失敗(誤り.to_string()))?
        .into_rgba8();
    let (幅, 高さ) = デコード画像.dimensions();

    Ok(Some(テクスチャデータ {
        幅,
        高さ,
        rgba8: デコード画像.into_raw(),
    }))
}

fn 画像バイト列を取り出す(
    文書: &開いた文書,
    画像: &gltf::image::Image<'_>,
) -> Result<Vec<u8>, アセットエラー> {
    match 画像.source() {
        gltf::image::Source::Uri { uri, .. } => {
            if uri.starts_with("data:") {
                return Err(アセットエラー::未対応画像形式);
            }
            外部ファイルを読む(&文書.基準ディレクトリ, uri)
        }
        gltf::image::Source::View { view, .. } => {
            let バッファ = 文書
                .バッファ一覧
                .get(view.buffer().index())
                .ok_or(アセットエラー::未対応画像形式)?;
            let 開始 = view.offset();
            let 終了 = 開始 + view.length();
            バッファ
                .get(開始..終了)
                .map(<[u8]>::to_vec)
                .ok_or(アセットエラー::未対応画像形式)
        }
    }
}
