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
    #[error("--lod-probe-step引数が不正だった: {0}")]
    LOD探査刻み不正(String),
    #[error("--blend引数が不正だった: {0}")]
    ブレンド不正(String),
    #[error("ストリーミング上限の引数が不正だった: {0}")]
    ストリーミング上限不正(String),
    #[error("--global-offset引数が不正だった: {0}")]
    大域ずらし量不正(String),
    #[error("--camera-nudge引数が不正だった: {0}")]
    カメラずれ不正(String),
    #[error("--camera-pitch引数が不正だった: {0}")]
    カメラ俯角不正(String),
    #[error("--camera-yaw引数が不正だった: {0}")]
    カメラ方位不正(String),
    #[error("--time-of-day引数が不正だった: {0}")]
    一日内時刻不正(String),
    #[error("--time-scale引数が不正だった: {0}")]
    時間倍率不正(String),
    #[error("--lod-crack-pair引数が不正だった: {0}")]
    Lod継ぎ目検査不正(String),
    #[error("--streaming-preload-radius引数が不正だった: {0}")]
    先読み半径不正(String),
    #[error("--report-sky-pixel引数が不正だった: {0}")]
    空代表画素不正(String),
    #[error("--shadow-resolution引数が不正だった: {0}")]
    影の一辺解像度不正(String),
    #[error("--caster-margin引数が不正だった: {0}")]
    キャスター余白不正(String),
    #[error("--max-shadow-distance引数が不正だった: {0}")]
    最大影距離不正(String),
}
