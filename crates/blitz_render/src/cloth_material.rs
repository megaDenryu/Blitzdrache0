//! 布シミュレーションのGPU境界型(判断52・54)。blitz_simが生成したバイト列・一覧を検証付きで
//! 保持する。バイト列のレイアウトはshaders/cloth_step.slang冒頭のバインディング表と一致する
//! (粒子32バイト・拘束の引数16バイト/拘束・ラグランジュ乗数4バイト/拘束・目標拘束の引数8バイト/拘束・目標位置16バイト/拘束)。
//! 色の区間の検証は`coloring`、距離拘束の内容の検証は`constraint_check`、目標拘束の内容の検証は`target_check`、粒子の逆質量の検証は`particle_check`、
//! 長さと添字の検証は`length_check`、拒む入力は`error`、GPUから写した粒子と乗数の器は`readback`、刻み幅の型は`time_step_width`、
//! 布の空間定数と自己衝突の別は`constants`、目標の更新対応とその2つの添字の型は`target_update_pair`・`target_index`・`skin_vertex_index`が持つ。

mod coloring;
mod constants;
mod constraint_check;
mod error;
mod length_check;
mod particle_check;
mod readback;
mod skin_vertex_index;
mod target_check;
mod target_index;
#[cfg(test)]
mod target_tests;
mod target_update_pair;
#[cfg(test)]
mod target_update_pair_tests;
#[cfg(test)]
mod test_fixtures;
#[cfg(test)]
mod tests;
mod time_step_width;

pub use coloring::{布の彩色の区間, 布の彩色の色数上限};
pub use constants::{布の自己衝突, 布定数};
pub use error::布素材エラー;
pub use readback::布の読み戻し;
pub use skin_vertex_index::スキン頂点添字;
pub use target_index::布の目標拘束添字;
pub use target_update_pair::目標の更新対応;
pub use time_step_width::布の刻み幅;

use length_check::{添字を検証する, 長さを検証する};

/// 布の拘束の1刻みの反復回数。GPUは色ごとのディスパッチと目標拘束のディスパッチをこの回数繰り返し、CPUの参照計算は同じ拘束の並びをこの回数回す。
pub const 布の拘束の反復回数: u32 = 4;

/// 布素材を組む材料。生成の口が受け取る値の束であり、検証はまだ済んでいない。
/// 固定・アタッチ・掴みは目標拘束(判断6)であり、粒子の逆質量は物性のまま全粒子が正である。
#[derive(Debug, Clone)]
pub struct 布素材の材料 {
    pub 粒子バイト列: Vec<u8>,
    pub 拘束の引数バイト列: Vec<u8>,             // 彩色で並べ替えた順の距離拘束(a添字・b添字・静止長・コンプライアンス)
    pub ラグランジュ乗数の初期バイト列: Vec<u8>, // 距離拘束と目標拘束の合計ぶん、拘束ごと4バイト。刻みの開始でGPUが零へ戻す
    pub 色の区間一覧: Vec<布の彩色の区間>,
    pub 目標拘束の引数バイト列: Vec<u8>, // 目標拘束ごと8バイト(粒子添字・コンプライアンス)。世界固定点・アタッチ・掴みの枠
    pub 目標位置の初期バイト列: Vec<u8>, // 目標拘束ごと16バイト(目標位置xyz・有効)。動く目標と介入がGPUで書き換える
    pub 目標の更新対応一覧: Vec<目標の更新対応>, // 反復の前にスキン済み頂点の位置をその目標へ写す
    pub インデックス一覧: Vec<u32>,
    pub 一辺粒子数: u32,
    pub 定数: 布定数,
}

/// 布1枚ぶんのGPU入力。生成が距離拘束と目標拘束の内容(端点の範囲・両端が別の粒子・同じ色で端点を共有しない・定義域)と
/// 粒子の逆質量が正であることと、目標の更新対応の目標拘束添字が範囲内で一意であることまで検証する。
/// 目標の更新対応のスキン頂点添字の検証は、スキン済み頂点数を知るレンダラー生成時に行う。
#[derive(Debug, Clone)]
pub struct 布素材 {
    pub(crate) 粒子バイト列: Vec<u8>,
    pub(crate) 拘束の引数バイト列: Vec<u8>,
    pub(crate) ラグランジュ乗数の初期バイト列: Vec<u8>,
    pub(crate) 色の区間一覧: Vec<布の彩色の区間>,
    pub(crate) 目標拘束の引数バイト列: Vec<u8>,
    pub(crate) 目標位置の初期バイト列: Vec<u8>,
    pub(crate) 目標の更新対応一覧: Vec<目標の更新対応>,
    pub(crate) インデックス一覧: Vec<u32>,
    pub(crate) 粒子数: u32,
    pub(crate) 拘束の数: u32,
    pub(crate) 目標拘束の数: u32,
    pub(crate) 一辺粒子数: u32,
    pub(crate) 定数: 布定数,
}

impl 布素材 {
    pub fn 生成する(材料: 布素材の材料) -> Result<Self, 布素材エラー> {
        if 材料.一辺粒子数 < 2 {
            return Err(布素材エラー::一辺粒子数不足(材料.一辺粒子数));
        }
        let 粒子数 = 材料.一辺粒子数 * 材料.一辺粒子数;
        let 粒子数usize = usize::try_from(粒子数).unwrap_or_else(|_| panic!("粒子数がusizeに収まらない"));
        長さを検証する("粒子バイト列", 材料.粒子バイト列.len(), 粒子数usize * particle_check::粒子1件のバイト数)?;
        particle_check::逆質量を検証する(&材料.粒子バイト列)?;
        let 拘束の数 = coloring::拘束の数を読む(&材料.拘束の引数バイト列)?;
        let 目標拘束の数 = target_check::目標拘束の内容を検証する(&材料.目標拘束の引数バイト列, &材料.目標位置の初期バイト列, 粒子数)?;
        長さを検証する(
            "ラグランジュ乗数の初期バイト列",
            材料.ラグランジュ乗数の初期バイト列.len(),
            (拘束の数.配列の長さ() + 目標拘束の数.配列の長さ()) * 4,
        )?;
        coloring::色の区間を検証する(&材料.色の区間一覧, 拘束の数)?;
        constraint_check::拘束の内容を検証する(&材料.拘束の引数バイト列, 粒子数, &材料.色の区間一覧)?;
        for &添字 in &材料.インデックス一覧 {
            添字を検証する("インデックス一覧", 添字, 粒子数)?;
        }
        target_check::更新対応を検証する(&材料.目標の更新対応一覧, 目標拘束の数)?;
        Ok(Self {
            粒子バイト列: 材料.粒子バイト列,
            拘束の引数バイト列: 材料.拘束の引数バイト列,
            ラグランジュ乗数の初期バイト列: 材料.ラグランジュ乗数の初期バイト列,
            色の区間一覧: 材料.色の区間一覧,
            目標拘束の引数バイト列: 材料.目標拘束の引数バイト列,
            目標位置の初期バイト列: 材料.目標位置の初期バイト列,
            目標の更新対応一覧: 材料.目標の更新対応一覧,
            インデックス一覧: 材料.インデックス一覧,
            粒子数,
            拘束の数: 拘束の数.値(),
            目標拘束の数: 目標拘束の数.値(),
            一辺粒子数: 材料.一辺粒子数,
            定数: 材料.定数,
        })
    }
}
