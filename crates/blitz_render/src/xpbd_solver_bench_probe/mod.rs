//! XPBDの並列方式の計測(Issue #35)の公開入口。担当するのは「ウィンドウを持たないGPUで、1つの並列方式で拘束グラフを
//! 刻み数ぶん反復し、位置とラグランジュ乗数と刻みごとのGPU時間を生の値で返す」ことである。
//! 呼び出し元は計測の報告(`blitz_app`)だけであり、本番の布の経路はここを通らない。
//!
//! 生の4成分と生の単精度で返すのは、検査が非有限値や発散を観測できなければならないためである。
//! 刻み1本を1回の送信にし、送信ごとにフェンスで待ってタイムスタンプを読む。パス別GPU時間の計器は提示のフレームと同じ
//! 実装(`vulkan::gpu_timing`)であり、刻みが提示のフレームに当たる。
//! 参照: `_doc/設計/XPBD共通拘束基盤.md`「判断7」「判断9」。

mod material;
mod shader_set;

pub use material::{XPBD並列方式, XPBD彩色の区間, XPBD計測素材, XPBD計測素材エラー};
pub use shader_set::XPBDシェーダー一式;

use crate::error::レンダラーエラー;
use crate::gpu_pass_timing::{パス時間の分布, フレーム別の標本};
use crate::validation_counter::検証層の状況;
use crate::vulkan::xpbd_bench;

/// 1つの刻みでGPUの定数へ書く値。加速度による変位は(重力+外力)×刻み幅²であり、刻みごとに外力の有無で変わる。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XPBD計測の刻みの定数 {
    pub 加速度による変位: [f32; 3],
    pub 刻み幅の2乗の逆数: f32,
}

/// 計測の条件。刻みの定数の並びの長さが刻み数である。
#[derive(Debug, Clone)]
pub struct XPBD計測の条件 {
    pub 方式: XPBD並列方式,
    pub 反復回数: u32,
    pub 刻みの定数一覧: Vec<XPBD計測の刻みの定数>,
}

/// 走らせて読み戻した結果。並びは素材の点と拘束の並びそのものである。
pub struct XPBD計測の読み戻し {
    pub 位置: Vec<[f32; 4]>,
    pub ラグランジュ乗数: Vec<f32>,
    pub 刻み別のgpu時間: Vec<フレーム別の標本>,
    pub gpu時間の分布一覧: Vec<(&'static str, パス時間の分布)>,
    pub バッファの合計バイト数: u64,
    pub 一刻みのディスパッチ数: u32,
    pub 検証層の状況: 検証層の状況,
    pub 検証件数: u64,
}

/// 一刻みの合計を表す合成区間の名前。読む側(`blitz_app`の報告と`xtask`の計測)がこの語で区間を引く。
pub fn 一刻みの合計の区間名() -> &'static str {
    xpbd_bench::pass_names::一刻みの合計
}

pub fn xpbdの並列方式をgpuで走らせて読み戻す(
    素材: &XPBD計測素材,
    条件: &XPBD計測の条件,
    シェーダー: &XPBDシェーダー一式,
) -> Result<XPBD計測の読み戻し, レンダラーエラー> {
    let (結果, 観測) = xpbd_bench::走らせて読み戻す(素材, 条件, シェーダー)?;
    Ok(XPBD計測の読み戻し {
        位置: 結果.位置,
        ラグランジュ乗数: 結果.ラグランジュ乗数,
        刻み別のgpu時間: 結果.刻み別のgpu時間,
        gpu時間の分布一覧: 結果.gpu時間の分布一覧,
        バッファの合計バイト数: 結果.バッファの合計バイト数,
        一刻みのディスパッチ数: 結果.一刻みのディスパッチ数,
        検証層の状況: 観測.状況,
        検証件数: 観測.件数,
    })
}
