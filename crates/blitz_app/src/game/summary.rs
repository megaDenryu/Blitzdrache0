//! 終了時の報告へ渡すゲームの進行の要約を所有する。値だけを運ぶ器であり、表示の仕方は`reports/game.rs`が決める。

/// そのプロセスがゲームをどこまで回したかの要約。
pub(crate) struct ゲーム進行の要約 {
    pub(crate) ゲームの表示名: &'static str,
    /// ゲーム更新を何回行ったか。1刻みは描画フレーム1回ぶんであるため、ゲームを回した描画フレーム数と一致する。
    pub(crate) ゲーム更新の回数: u32,
    pub(crate) 最後の進行段階の呼び名: &'static str,
    pub(crate) 到達済みの目的地数: usize,
    pub(crate) 目的地の総数: usize,
}
