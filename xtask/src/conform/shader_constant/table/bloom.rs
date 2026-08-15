//! 光のにじみピラミッドの段の刻み方と、にじみへ光が入り始める明るさの正本と、その写しの対応。
//! 写しはxtaskの中にある。
//!
//! 遠景の検収(`cargo xtask distant-view --judge`)は、後処理ありの対に残る色差が「にじみが1画素の変化を
//! 運べる範囲」に収まっていることを課す。その範囲は段の数と段ごとのテクセルの大きさから導くため、
//! 検収は段の刻み方を知っている必要がある。xtaskは外部のクレートへ依存しない方針でありblitz_renderを
//! 読めないため、値を持ち直している。
//!
//! 段数上限が正本で増えると、写しのままの検収は上限を実際より小さく見積もる。滲みの届く画素が
//! 足跡の外と判定され、検収は落ちる。見逃しでなく失敗になる向きではあるが、そのとき落ちた理由が
//! 「にじみの実装が変わった」ことだと分かるように、ここが機械的に結ぶ。

use super::定数の組;

const 段の正本: &str = "crates/blitz_render/src/vulkan/bloom_targets.rs";
const 段の写し: &str = "xtask/src/distant_view/judgment/bloom_footprint.rs";
const 明るさの正本: &str = "shaders/bloom_filters.slang";
const 明るさの写し: &str = "xtask/src/distant_view/judgment/bloom_source.rs";

pub(super) const 定数一覧: [定数の組; 4] = [
    定数の組 {
        正本パス: 段の正本,
        正本の前置き: "pub(crate) const 段数上限: usize = ",
        写しパス: 段の写し,
        写しの前置き: "const 段数上限: usize = ",
    },
    定数の組 {
        正本パス: 段の正本,
        正本の前置き: "const 最小辺PX: u32 = ",
        写しパス: 段の写し,
        写しの前置き: "const 最小辺PX: usize = ",
    },
    定数の組 {
        正本パス: 明るさの正本,
        正本の前置き: "float threshold = ",
        写しパス: 明るさの写し,
        写しの前置き: "const にじみが光を全部通す明るさ: f32 = ",
    },
    定数の組 {
        正本パス: 明るさの正本,
        正本の前置き: "float knee = ",
        写しパス: 明るさの写し,
        写しの前置き: "const にじみの入り口をなだらかにする幅: f32 = ",
    },
];
