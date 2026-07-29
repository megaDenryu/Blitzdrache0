//! 大気LUTのGPU読み戻し報告。ウィンドウもスワップチェーンも作らず、ヘッドレスのGPUで2枚のLUTを焼いて読み戻し、
//! CPU正本との差と全体の統計を機械可読な行として標準出力へ出す。
//! これが大気LUT生成シェーダーの本番外の呼び出し元であり、判定は`cargo xtask atmosphere-lut`がこの出力を読んで行う。
//!
//! 同じ入力で2回焼いて突き合わせるのは、決定性の契約(同一GPU実装・同一入力で同一出力)を確かめるためである。

mod report_error;
mod rows;
mod statistics;

use std::process::ExitCode;

use blitz_engine::sky::atmosphere::大気媒体方針;
use blitz_render::atmosphere::{大気LUT解像度, 透過率表};

use report_error::大気LUT報告エラー;

pub(crate) fn 大気lut表を出す() -> ExitCode {
    match 報告する() {
        Ok(()) => ExitCode::SUCCESS,
        Err(誤り) => {
            eprintln!("大気LUT報告に失敗した: {誤り}");
            ExitCode::FAILURE
        }
    }
}

fn 報告する() -> Result<(), 大気LUT報告エラー> {
    let シェーダー = crate::embedded_shaders::埋め込み大気lutシェーダーを生成する()?;
    let 媒体 = crate::atmosphere_medium::大気散乱媒体へ写す(&大気媒体方針::地球標準())?;
    let 解像度 = 大気LUT解像度::既定値();
    println!(
        "大気LUT報告 透過率幅={} 透過率高さ={} 多重散乱一辺={}",
        解像度.透過率の幅(),
        解像度.透過率の高さ(),
        解像度.多重散乱の一辺()
    );

    let 一回目 = blitz_render::atmosphere_lut_probe::大気lutをgpuで焼いて読み戻す(&媒体, 解像度, &シェーダー)?;
    let 二回目 = blitz_render::atmosphere_lut_probe::大気lutをgpuで焼いて読み戻す(&媒体, 解像度, &シェーダー)?;
    rows::再現性を出す(&一回目, &二回目);

    let 透過率表 = 透過率表::焼く(媒体.消散(), 解像度)?;
    statistics::透過率の統計を出す(一回目.透過率());
    rows::透過率の代表テクセルを出す(&媒体, 解像度, 一回目.透過率())?;
    statistics::多重散乱の統計を出す(一回目.多重散乱());
    rows::多重散乱の代表テクセルを出す(&媒体, &透過率表, 解像度, 一回目.多重散乱())?;
    Ok(())
}
