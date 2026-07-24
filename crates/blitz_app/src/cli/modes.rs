//! CLIで選ぶ表示物の種別: 布シミュレーションの方式と粒子系の検証対象。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 布モード {
    なし,
    吊るし布,
    マント,
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
