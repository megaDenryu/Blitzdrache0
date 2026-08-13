//! 逆向き深度の固定検収が使う世界寸法の写しと、本番のストリーミング格子の正本を結ぶ台帳。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_app/src/app/streaming.rs",
    正本の前置き: "pub(super) const 一辺メートル: f64 = ",
    写しパス: "xtask/src/reverse_depth/run.rs",
    写しの前置き: "const チャンクの一辺メートル: f32 = ",
}];
