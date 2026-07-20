//! バッファ実体の解決: glTFのBin(glb埋め込み)/Uri(外部.binファイル)を
//! 実バイト列へ変換する。添字はglTFのバッファ添字と一致させ、Vecに並べる。
//! 外部ファイルを読んだ場合はそのパスも集めて返す(参照ファイル一覧・ホットリロード監視用)。

use std::path::{Path, PathBuf};

use crate::asset::error::アセットエラー;

use super::file::外部ファイルを読む;

pub(super) fn バッファ一覧を解決する(
    文書: &gltf::Document,
    基準ディレクトリ: &Path,
    埋め込みバイナリ: Option<Vec<u8>>,
) -> Result<(Vec<Vec<u8>>, Vec<PathBuf>), アセットエラー> {
    let mut 結果 = Vec::with_capacity(文書.buffers().len());
    let mut 参照パス一覧 = Vec::new();
    for バッファ in 文書.buffers() {
        let データ = match バッファ.source() {
            gltf::buffer::Source::Bin => 埋め込みバイナリ
                .clone()
                .ok_or(アセットエラー::未対応バッファ形式)?,
            gltf::buffer::Source::Uri(uri) => {
                if uri.starts_with("data:") {
                    return Err(アセットエラー::未対応バッファ形式);
                }
                let (バイト列, パス) = 外部ファイルを読む(基準ディレクトリ, uri)?;
                参照パス一覧.push(パス);
                バイト列
            }
        };
        結果.push(データ);
    }
    Ok((結果, 参照パス一覧))
}
