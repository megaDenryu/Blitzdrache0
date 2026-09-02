//! XPBDの並列方式の計測の報告(Issue #35)。ウィンドウもスワップチェーンも作らず、ウィンドウなし実行のGPUで1つの並列方式を
//! 1つの拘束グラフで刻み数ぶん反復し、位置と乗数の読み戻しをCPUの参照計算(正典式)と突き合わせて機械可読な行として標準出力へ出す。
//! 判定は`cargo xtask xpbd-solver-bench`がこの出力を読んで行い、ここは事実の行だけを出す。
//!
//! 同じ入力で2回走らせて突き合わせるのは、方式ごとの決定性(グラフ彩色と二段階はビット一致、原子加算は原子加算の順で変わりうる)を
//! 事実として残すためである。CPUの参照計算との突き合わせは短い実行(比較の刻み数)で別に走らせる。長い実行では単精度の演算順の
//! 差が吊るした布の揺れで増幅され、同じ式であっても軌道が離れるためであり、長い実行は拘束違反の二乗平均平方根で突き合わせる。
//! 題材の組み立ては`fixture`、加速度の予定は`acceleration_schedule`、参照計算は`cpu_reference`、
//! 突き合わせは`comparison`、指紋は`sha256`、行の綴りは`lines`が持つ。

mod acceleration_schedule;
mod comparison;
mod cpu_reference;
mod fixture;
mod lines;
mod sha256;
#[cfg(test)]
mod sha256_tests;

use std::process::ExitCode;

use blitz_render::xpbd_solver_bench_probe::{XPBD計測の条件, xpbdの並列方式をgpuで走らせて読み戻す};

use crate::cli::XPBD並列方式計測の指定;
use crate::error::起動エラー;

pub(crate) fn xpbd並列方式の計測表を出す(指定: &XPBD並列方式計測の指定) -> ExitCode {
    match 報告する(指定) {
        Ok(()) => ExitCode::SUCCESS,
        Err(誤り) => {
            eprintln!("XPBDの並列方式の計測の報告に失敗した: {誤り}");
            ExitCode::FAILURE
        }
    }
}

fn 報告する(指定: &XPBD並列方式計測の指定) -> Result<(), 起動エラー> {
    let シェーダー = crate::embedded_xpbd_shaders::埋め込みxpbdシェーダーを生成する()?;
    let 題材 = fixture::題材を作る(指定)?;
    let 刻み幅 = acceleration_schedule::刻み幅()?;
    let 条件 = XPBD計測の条件 {
        方式: 指定.方式,
        反復回数: 指定.反復回数,
        刻みの定数一覧: acceleration_schedule::刻みの定数一覧(指定.刻み数, 刻み幅),
    };
    let 比較の条件 = XPBD計測の条件 {
        方式: 指定.方式,
        反復回数: 指定.反復回数,
        刻みの定数一覧: acceleration_schedule::刻みの定数一覧(指定.比較の刻み数, 刻み幅),
    };
    let 素材 = 題材.素材にする()?;
    lines::題材を出す(指定, &題材);
    let 一回目 = xpbdの並列方式をgpuで走らせて読み戻す(&素材, &条件, &シェーダー)?;
    let 二回目 = xpbdの並列方式をgpuで走らせて読み戻す(&素材, &条件, &シェーダー)?;
    let 比較用 = xpbdの並列方式をgpuで走らせて読み戻す(&素材, &比較の条件, &シェーダー)?;
    let 参照 = cpu_reference::参照計算を回す(&題材, 指定.方式, 指定.刻み数, 指定.反復回数, 刻み幅)?;
    let 比較用の参照 = cpu_reference::参照計算を回す(&題材, 指定.方式, 指定.比較の刻み数, 指定.反復回数, 刻み幅)?;
    let 突き合わせ = comparison::突き合わせる(&題材, &一回目, &二回目, &参照);
    let 短い突き合わせ = comparison::突き合わせる(&題材, &比較用, &比較用, &比較用の参照);
    lines::検証を出す(&[&一回目, &二回目, &比較用]);
    lines::再現性を出す(&突き合わせ);
    lines::収束を出す(&突き合わせ, &参照);
    lines::参照との差を出す(指定.比較の刻み数, &短い突き合わせ);
    lines::資源を出す(指定.方式, &一回目);
    crate::reports::gpu_time_table::表示する(&一回目.gpu時間の分布一覧);
    crate::reports::gpu_frame_samples::表示する(&一回目.刻み別のgpu時間);
    Ok(())
}
