//! 開発用UIパネルに表示する統計値の束(判断34)。

pub(crate) struct 開発UI統計 {
    /// パス名ごとの移動平均GPU時間(ミリ秒、判断30)。計測非対応なら空。
    pub(crate) パス別gpu時間: Vec<(&'static str, blitz_render::gpu_pass_timing::パス時間の分布)>,
    /// 直近フレームのCPU側フレーム時間の移動平均(ミリ秒)。
    pub(crate) フレーム時間ms: f64,
    /// これまでのvalidationエラー・警告合計件数。
    pub(crate) validation件数: u64,
}
