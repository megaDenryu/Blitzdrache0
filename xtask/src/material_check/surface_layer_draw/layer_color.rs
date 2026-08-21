//! 層ごとのタイルの純色と、絵からその色を読み出す工程。
//!
//! 純色の値は`crates/blitz_asset_compiler/examples/generate_source_assets/surface_layer_tiles.rs`の写しである。
//! xtaskはpngを復号する手立てを持たないため値を持ち直す。写しがずれると、この検収は「層のタイルが出ていない」と
//! 言い張るようになるため、生成器の色を変えるときはここも変える。
//!
//! ライティングもポスト処理も切った条件では、重みが1つの層へ満量で寄った面の画素は、その層のタイルの
//! 符号値そのものになる。標本と伝達関数の往復で1段動きうるため、成分ごとの差の許容を2段まで認める。

use crate::acceptance::{画素の横位置, 画素の縦位置, 読み戻し画像};

/// 実在する色として認める成分ごとの差。伝達関数の丸めが動く幅である1段ぶんに、読み戻しの経路が持ちうる同じ幅を足す。
pub(super) const 許容する成分差: u8 = 2;

/// 層番号の昇順に並べた、その層のタイルの純色。層0が草、層1が泥、層2が岩、層3が砂である。
const 層ごとの純色: [[u8; 3]; 4] = [[60, 140, 60], [130, 90, 50], [140, 140, 140], [200, 180, 110]];

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct 層の色 {
    層番号: usize,
}

impl 層の色 {
    pub(super) const 草: Self = Self { 層番号: 0 };
    pub(super) const 泥: Self = Self { 層番号: 1 };
    pub(super) const 岩: Self = Self { 層番号: 2 };
    pub(super) const 砂: Self = Self { 層番号: 3 };

    pub(super) fn 呼び名(self) -> String {
        format!("層{}", self.層番号)
    }

    pub(super) fn 純色(self) -> [u8; 3] {
        層ごとの純色[self.層番号]
    }

    pub(super) fn 画素が一致するか(self, 画素: [u8; 3]) -> bool {
        let 純色 = self.純色();
        (0..3).all(|成分| 画素[成分].abs_diff(純色[成分]) <= 許容する成分差)
    }
}

/// 絵の四隅の1つ。位置を千分率で持つのは、読み戻し画像の寸法が窓の大きさで変わるためである。
#[derive(Clone, Copy)]
pub(super) struct 隅の位置 {
    横千分率: usize,
    縦千分率: usize,
}

impl 隅の位置 {
    /// 四隅の並び。左上・右上・左下・右下の順であり、判定はこの順で期待する層を並べる。
    pub(super) const 四隅: [Self; 4] = [
        Self {
            横千分率: 80, 縦千分率: 80
        },
        Self {
            横千分率: 920, 縦千分率: 80
        },
        Self {
            横千分率: 80, 縦千分率: 920
        },
        Self {
            横千分率: 920,
            縦千分率: 920,
        },
    ];

    pub(super) fn 呼び名(self) -> String {
        let 横 = if self.横千分率 < 500 { "左" } else { "右" };
        let 縦 = if self.縦千分率 < 500 { "上" } else { "下" };
        format!("画面の{縦}{横}")
    }

    pub(super) fn 画素を読む(self, 画像: &読み戻し画像) -> [u8; 3] {
        let 横 = 千分率を画素へ(画像.幅().画素数(), self.横千分率);
        let 縦 = 千分率を画素へ(画像.高さ().画素数(), self.縦千分率);
        画像.座標の画素(画素の横位置::生成する(横), 画素の縦位置::生成する(縦))
    }
}

/// その層の純色を持たない画素の数。地面が画面を覆う構図で、混ざり物がどれだけ出たかを数える。
pub(super) fn 純色でない画素数を数える(画像: &読み戻し画像, 層: 層の色) -> u64 {
    画像
        .画素列()
        .filter(|画素| !層.画素が一致するか([画素[0], 画素[1], 画素[2]]))
        .count()
        .try_into()
        .unwrap_or(u64::MAX)
}

fn 千分率を画素へ(画素数: usize, 千分率: usize) -> usize {
    (画素数 * 千分率 / 1000).min(画素数.saturating_sub(1))
}
