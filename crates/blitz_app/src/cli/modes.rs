//! CLIで選ぶ表示物の種別: 布シミュレーションの方式と粒子系の検証対象。

use super::cloth_bending_compliance::布の曲げのコンプライアンス指定;
use super::cloth_compliance::布のコンプライアンス指定;
use super::cloth_reference_shape::参照比較の題材の形;

/// `XPBD参照比較`は、自己衝突に触れない条件で吊るした布をGPUで進め、終了時にCPUの参照計算と突き合わせる検収の方式である
/// (Issue #36「検証」)。コンプライアンスは構造とせん断の両方へ同じ値を与え、曲げのコンプライアンスは曲げ拘束へ与える(Issue #38)。床の下の固定点を持つ題材は、
/// 下端の1粒子を床の下の目標へ固定して、床の後の目標拘束の最終の成立(Issue #37の是正)をGPUとCPUで突き合わせる。題材の形は布の敷き方と固定の別である。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum 布モード {
    なし,
    吊るし布,
    マント,
    XPBD参照比較 {
        コンプライアンス: 布のコンプライアンス指定,
        床の下の固定点: 参照比較の床の下の固定点,
        曲げのコンプライアンス: 布の曲げのコンプライアンス指定,
        題材の形: 参照比較の題材の形,
    },
}

/// 参照比較の布が、目標が床の下にある世界固定点を1本持つかどうか。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 参照比較の床の下の固定点 {
    持つ,
    持たない,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 粒子表示モード {
    なし,
    粒子トイ,
    表面流,
    Sph512,
    Sph1024,
    Sph2048,
}
