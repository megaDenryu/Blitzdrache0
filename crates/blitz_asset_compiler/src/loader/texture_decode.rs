//! gltf::Textureから画像バイト列を読み取りRGBA8へデコードする共通処理。
//! ベースカラー・metallicRoughness・法線マップの3種のテクスチャ抽出がこの
//! 1関数を共有する（判断23）。

use std::path::PathBuf;

use blitz_engine::テクスチャデータ;

use crate::error::アセットコンパイルエラー;

use super::document::開いた文書;
use super::file::外部ファイルを読む;

/// 戻り値の2番目は、外部画像ファイルを読んだ場合のそのパス(参照ファイル一覧用)。
pub(super) fn デコードする(
    文書: &開いた文書,
    テクスチャ: &gltf::Texture<'_>,
) -> Result<(テクスチャデータ, Option<PathBuf>), アセットコンパイルエラー> {
    let 画像 = テクスチャ.source();
    let (バイト列, 参照パス) = 画像バイト列を取り出す(文書, &画像)?;

    let デコード画像 = image::load_from_memory(&バイト列)
        .map_err(|誤り| アセットコンパイルエラー::画像デコード失敗(誤り.to_string()))?
        .into_rgba8();
    let (幅, 高さ) = デコード画像.dimensions();

    Ok((
        テクスチャデータ {
            幅,
            高さ,
            rgba8: デコード画像.into_raw(),
        },
        参照パス,
    ))
}

fn 画像バイト列を取り出す(
    文書: &開いた文書,
    画像: &gltf::image::Image<'_>,
) -> Result<(Vec<u8>, Option<PathBuf>), アセットコンパイルエラー> {
    match 画像.source() {
        gltf::image::Source::Uri { uri, .. } => {
            if uri.starts_with("data:") {
                return Err(アセットコンパイルエラー::未対応画像形式);
            }
            let (バイト列, パス) = 外部ファイルを読む(&文書.基準ディレクトリ, uri)?;
            Ok((バイト列, Some(パス)))
        }
        gltf::image::Source::View { view, .. } => {
            let バッファ = 文書
                .バッファ一覧
                .get(view.buffer().index())
                .ok_or(アセットコンパイルエラー::未対応画像形式)?;
            let 開始 = view.offset();
            let 終了 = 開始 + view.length();
            let バイト列 = バッファ
                .get(開始..終了)
                .map(<[u8]>::to_vec)
                .ok_or(アセットコンパイルエラー::未対応画像形式)?;
            Ok((バイト列, None))
        }
    }
}
