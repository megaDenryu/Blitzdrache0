//! バッファ実体の解決: glTFのBin(glb埋め込み)/Uri(外部.binファイル)を
//! 実バイト列へ変換する。添字はglTFのバッファ添字と一致させ、Vecに並べる。

use std::path::Path;

use crate::asset::error::アセットエラー;

use super::file::外部ファイルを読む;

pub(super) fn バッファ一覧を解決する(
    文書: &gltf::Document,
    基準ディレクトリ: &Path,
    埋め込みバイナリ: Option<Vec<u8>>,
) -> Result<Vec<Vec<u8>>, アセットエラー> {
    let mut 結果 = Vec::with_capacity(文書.buffers().len());
    for バッファ in 文書.buffers() {
        let データ = match バッファ.source() {
            gltf::buffer::Source::Bin => 埋め込みバイナリ
                .clone()
                .ok_or(アセットエラー::未対応バッファ形式)?,
            gltf::buffer::Source::Uri(uri) => {
                if uri.starts_with("data:") {
                    return Err(アセットエラー::未対応バッファ形式);
                }
                外部ファイルを読む(基準ディレクトリ, uri)?
            }
        };
        結果.push(データ);
    }
    Ok(結果)
}
