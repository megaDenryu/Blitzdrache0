//! 派生表現報告の行の形。担当するのは、報告が運ぶ値の型だけである。読み取りの手順は`parse`が持つ。

/// 定数の環境1つと1つの段についての、解析解との最大相対誤差。拡散の行は段を0で埋める。
pub(super) struct 定数環境行 {
    pub(super) 放射輝度緑: f64,
    pub(super) 段: u32,
    pub(super) 最大相対誤差: f64,
}

pub(super) struct 統計行 {
    pub(super) 種別: String,
    pub(super) テクセル数: usize,
    pub(super) 非有限: usize,
    pub(super) 負: usize,
    pub(super) 予約が非零: usize,
}

pub(super) struct 分散行 {
    pub(super) 段: u32,
    pub(super) 分散: f64,
}

pub(super) struct 面境界行 {
    pub(super) 種別: String,
    pub(super) 段: u32,
    pub(super) 一辺: u32,
    pub(super) 稜線数: usize,
    pub(super) 最大相対差: f64,
}

pub(super) struct 代表行 {
    pub(super) 種別: String,
    pub(super) 段: u32,
    pub(super) 層: u32,
    pub(super) 横: u32,
    pub(super) 縦: u32,
    pub(super) 期待: [f64; 3],
    pub(super) 実測: [f64; 3],
}

pub(super) struct 反射率積分表行 {
    pub(super) 横: u32,
    pub(super) 縦: u32,
    pub(super) 期待: [f64; 2],
    pub(super) 実測: [f64; 2],
}

pub(super) struct 報告 {
    pub(super) 検証: Option<(String, usize)>,
    pub(super) 定数環境拡散: Vec<定数環境行>,
    pub(super) 定数環境鏡面: Vec<定数環境行>,
    pub(super) 統計: Vec<統計行>,
    pub(super) 分散: Vec<分散行>,
    pub(super) 最詳細段の不一致: Option<usize>,
    pub(super) 面境界: Vec<面境界行>,
    pub(super) 代表: Vec<代表行>,
    pub(super) 反射率積分表代表: Vec<反射率積分表行>,
}
