//! glTFファイルを開き、文書とバッファ実体を解決した中間表現を作る。
//! gltf型はこの構造体の内部にのみ現れ、asset公開APIには出さない。

use std::path::{Path, PathBuf};

use crate::asset::error::アセットエラー;

use super::buffer::バッファ一覧を解決する;

/// 解析済みのglTF文書と、解決済みのバッファ実体。ロード処理専用の中間表現。
/// `参照ファイル一覧` は主ファイルと外部バッファファイルのパス(この時点でテクスチャの
/// 外部ファイルは未確定のため含まない。呼び出し元の`loader::mod`が最終的に合成する)。
pub(super) struct 開いた文書 {
    pub(super) document: gltf::Document,
    pub(super) 基準ディレクトリ: PathBuf,
    pub(super) バッファ一覧: Vec<Vec<u8>>,
    pub(super) 参照ファイル一覧: Vec<PathBuf>,
}

/// .gltf(JSON+外部バッファ)と.glb(バイナリ埋め込み)の両方を読む。
/// 形式の判別は`gltf::Gltf::from_slice`のマジックバイト判定に委ねる。
pub(super) fn 文書を開く(パス: &Path) -> Result<開いた文書, アセットエラー> {
    let バイト列 = std::fs::read(パス).map_err(|誤り| アセットエラー::ファイル読込失敗(format!("{}: {誤り}", パス.display())))?;
    let gltf::Gltf { document, blob } = gltf::Gltf::from_slice(&バイト列).map_err(|誤り| アセットエラー::解析失敗(誤り.to_string()))?;

    let 基準ディレクトリ = パス.parent().map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("."));
    let (バッファ一覧, バッファ参照パス一覧) = バッファ一覧を解決する(&document, &基準ディレクトリ, blob)?;

    let mut 参照ファイル一覧 = vec![パス.to_path_buf()];
    参照ファイル一覧.extend(バッファ参照パス一覧);

    Ok(開いた文書 {
        document,
        基準ディレクトリ,
        バッファ一覧,
        参照ファイル一覧,
    })
}
