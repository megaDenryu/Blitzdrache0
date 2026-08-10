//! 着手順5の第4段階の検収入口。夜の多光源の世界と屋内の多光源の世界を本番の描画経路で走らせ、
//! 読み戻しの決定性・validationの指摘が0件であること・光の件数を振ったときのGPU時間と割り当ての統計を採る。
//!
//! 判定は3つである。同一の起動の中で撮った2枚がバイト単位で一致すること、どの実行もvalidationが0件であること、
//! 屋内の灯を消したときより灯を点けたときのほうが屋内が明るいことである。**GPU時間には判定を置かない。**
//! 枠の制定は実測を読んでから人が行う(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断h」)。
//!
//! 絵の合否も判定に含まれない。書き出したPNGを見て決めるのは親エージェントである。
//! 参照: `_doc/設計/クラスタ多光源と点光源の影.md`「検収戦略(判断i)」

mod assignment;
mod determinism;
mod gpu_time;
mod interior;
mod run;
mod scale;
mod summary;
mod world;

use std::path::PathBuf;
use std::process::ExitCode;

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
    world::夜の世界を用意する()?;
    world::屋内の世界を用意する()?;
    crate::release_build::計測用に構築する("cluster-lights")?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 夜の絵 = determinism::確かめる(&出力先)?;
    let 結果一覧 = scale::測る(&出力先)?;
    let 屋内 = interior::測る(&出力先)?;
    let 比 = 屋内.灯ありの平均輝度 / 屋内.灯なしの平均輝度;
    if 比 < interior::明るさ比の下限 {
        return Err(format!(
            "屋内の灯ありと灯なしの平均輝度の比{比:.2}が下限{}未満だった(灯あり{:.1}・灯なし{:.1}): 屋内へ点光源が届いていない",
            interior::明るさ比の下限,
            屋内.灯ありの平均輝度,
            屋内.灯なしの平均輝度
        ));
    }
    summary::表を表示する(&結果一覧);
    Ok(summary::要約を組む(&結果一覧, &屋内, &夜の絵))
}
