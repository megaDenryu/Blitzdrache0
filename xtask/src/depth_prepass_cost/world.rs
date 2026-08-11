//! 計測と検収が使う世界の起動引数。担当するのは、費用を測る起動と絵を撮る起動が同じ世界を同じ条件で開くことである。
//!
//! `indirect-cost`と同じ世界・同じ先読み半径・同じカメラ俯角にするのは、既に採ってある色パスの費用と並べて読めるようにするためである。
//! 別の構図で測ると、深度プリパスが足す費用の大きさを既存の値と比べる根拠が消える。
//! 時間再構成は`--no-taa`で外す。段3b以前に採った値と並べて読むため、パスを1本足した条件へ計測窓を変えない。

#[cfg(test)]
mod tests;

use super::schedule::実行条件;

/// 計測がこの世界を叩くときの起こし方。GPU時間の窓へcargoのビルド判定を混ぜないため、構築済みのリリース版を直に起こす。
pub(super) const 起こし方: crate::acceptance::アプリの起こし方 =
    crate::acceptance::アプリの起こし方::構築済みのリリース版を直に起動する;
const シーン名: &str = "terrain_origin";
const アセットルート: &str = "target/terrain_assets";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";

/// 世界とカメラを決める引数。条件によらず同じである。
///
/// 局所可視性補正を切るのは、3条件の目的が深度プリパスの費用対効果を分離することだからである。補正を宣言した
/// 世界は深度プリパスの方式を引き上げるため、切らないと条件Aの`使わない`が型付きの失敗になって条件列が成立しない。
/// 補正込みの費用は通常実行の計器で別に採る。
pub(super) fn 世界の引数() -> Vec<String> {
    [
        "--no-ssao",
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
        "--no-taa",
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
