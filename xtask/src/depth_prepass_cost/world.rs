//! 計測と検収が使う世界の起動引数。担当するのは、費用を測る起動と絵を撮る起動が同じ世界を同じ条件で開くことである。
//!
//! `indirect-cost`と同じ世界・同じ先読み半径・同じカメラ俯角にするのは、既に採ってある色パスの費用と並べて読めるようにするためである。
//! 別の構図で測ると、深度プリパスが足す費用の大きさを既存の値と比べる根拠が消える。

use super::schedule::実行条件;

pub(super) const 実行ファイル: &str = "target/release/blitz_app.exe";
const シーン名: &str = "terrain_origin";
const アセットルート: &str = "target/terrain_assets";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";

/// 世界とカメラを決める引数。条件によらず同じである。
pub(super) fn 世界の引数() -> Vec<String> {
    [
        "--scene",
        シーン名,
        "--asset-root",
        アセットルート,
        "--streaming",
        "--streaming-preload-radius",
        先読み半径,
        "--streaming-ram-limit",
        容量上限バイト,
        "--streaming-vram-limit",
        容量上限バイト,
        "--camera-pitch",
        カメラ俯角差分度,
    ]
    .iter()
    .map(|語| (*語).to_string())
    .collect()
}

pub(super) fn 条件の引数(条件: &実行条件) -> Vec<String> {
    vec!["--depth-prepass".to_string(), 条件.方式.起動指定の語().to_string()]
}

pub(super) fn 時刻の引数(一日内秒: Option<&String>) -> Vec<String> {
    一日内秒.map_or_else(Vec::new, |秒| vec!["--time-of-day".to_string(), 秒.clone()])
}
