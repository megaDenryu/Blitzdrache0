//! 時間の規律の基本刻みの正本と、その写しの対応。布のXPBD参照比較の検収は、報告に出た布の刻み幅が正本の基本刻みと一致することを
//! 判定するが、xtaskはblitz_appへ依存しないため値を持ち直す。写しがずれると、布だけ古い刻みで進む経路を検収が緑のまま通すため、ここが機械的に見る。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_app/src/game/step_seconds.rs",
    正本の前置き: "const 一刻みの秒数: f32 = ",
    写しパス: "xtask/src/cloth_xpbd_reference/judgment.rs",
    写しの前置き: "const 正本の基本刻みの秒: f64 = ",
}];
