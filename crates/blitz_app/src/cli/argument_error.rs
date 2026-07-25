//! CLI引数の解析で返す型付きエラー。どの引数がどう不正だったかの語彙を所有する。

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum 起動引数エラー {
    #[error("--frames引数が不正だった: {0}")]
    フレーム数不正(String),
    #[error("--shader-source引数が不正だった: {0}")]
    シェーダーソース不正(String),
    #[error("--scene引数が不正だった: {0}")]
    シーン名不正(String),
    #[error("--asset-root引数が不正だった: {0}")]
    アセットルート不正(String),
    #[error("--object-count引数が不正だった: {0}")]
    描画対象数不正(String),
    #[error("--dump-frame引数が不正だった: {0}")]
    フレームダンプ不正(String),
    #[error("--exposure引数が不正だった: {0}")]
    露出不正(String),
    #[error("--blend引数が不正だった: {0}")]
    ブレンド不正(String),
    #[error("ストリーミング上限の引数が不正だった: {0}")]
    ストリーミング上限不正(String),
}
