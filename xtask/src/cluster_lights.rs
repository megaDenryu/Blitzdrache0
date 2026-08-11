//! 着手順5の第4段階の検収入口。夜の多光源の世界と屋内の多光源の世界を本番の描画経路で走らせ、
//! 読み戻しの決定性・validationの指摘が0件であること・光の件数を振ったときのGPU時間と割り当ての統計を採る。
//!
//! 判定は4つである。同一の起動の中で撮った2枚がバイト単位で一致することと、そのとき出た割り当ての2行も一致すること、
//! どの実行もvalidationが0件であること、屋内の灯を消したときより灯を点けたときのほうが屋内が明るいことである。
//! **GPU時間には判定を置かない。**
//! 枠の制定は実測を読んでから人が行う(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断h」)。
//!
//! 絵の合否も判定に含まれない。書き出したPNGを見て決めるのは親エージェントである。
//! 参照: `_doc/設計/クラスタ多光源と点光源の影.md`「検収戦略(判断i)」

mod assignment;
mod determinism;
mod gpu_time;
#[cfg(test)]
mod gpu_time_tests;
mod interior;
mod interior_judgment;
mod scale;
mod summary;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::multi_light_world::world;

const 出力ディレクトリ: &str = "target/cluster_lights";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] cluster-lights成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] cluster-lights失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    world::夜の世界を用意する().map_err(|破れ| 破れ.to_string())?;
    world::屋内の世界を用意する().map_err(|破れ| 破れ.to_string())?;
    crate::release_build::計測用に構築する("cluster-lights").map_err(|破れ| 破れ.to_string())?;
    let 夜の実行環境 = world::夜の世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;
    let 屋内の実行環境 = world::屋内の世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;

    let 夜の絵 = determinism::確かめる(&夜の実行環境)?;
    let 結果一覧 = scale::測る(&夜の実行環境)?;
    let 屋内 = interior::測る(&屋内の実行環境)?;
    interior_judgment::明るさの比を判定する(屋内.灯ありの平均輝度, 屋内.灯なしの平均輝度)?;
    summary::表を表示する(&結果一覧);
    Ok(summary::要約を組む(&結果一覧, &屋内, &夜の絵))
}
