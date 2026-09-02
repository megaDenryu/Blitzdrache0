//! CLIで選ぶ表示物の種別: 布シミュレーションの方式と粒子系の検証対象。

use super::cloth_compliance::布のコンプライアンス指定;

/// `XPBD参照比較`は、床にも自己衝突にも触れない条件で吊るした布をGPUで進め、終了時にCPUの参照計算と突き合わせる検収の方式である
/// (Issue #36「検証」)。コンプライアンスは構造とせん断の両方へ同じ値を与える。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum 布モード {
    なし,
    吊るし布,
    マント,
    XPBD参照比較 {
        コンプライアンス: 布のコンプライアンス指定,
    },
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
