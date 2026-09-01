//! OW5第3段の検収入口。空を持つ地形世界を4つの代表時刻で描き、時刻を変えると空・照明・影・露出のすべてが
//! 決定的に変わることを画素で確かめる。
//!
//! 判定は9つある。同一時刻の2回描画がバイト一致すること、隣り合う時刻の間で空色・明部・暗部がそれぞれ変わること、
//! 夜明けと夕方の空平均色がバイト一致すること(太陽高度0で方位が東西反対という配置の対称性)、
//! 夜の地表が環境光だけの明るさに収まり黒へ落ちないこと、4時刻のどの空も飽和しないこと、
//! 太陽が画面外の対照との差分で太陽円盤の高輝度画素が円盤の見込み面積の帯に入ること、
//! 太陽円盤の明るさだけを変えた対で遠方環境の鍵と積んだ本数と地形の表面画素が動かないこと、
//! 4時刻の地形が破綻防止帯に収まること、全条件でvalidationが0件であることである。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「検証の設計」、`_doc/設計/放射輝度問い合わせ階層.md`「3-Ic-3bの実装」

mod draw;
mod error;
mod image_judgment;
#[cfg(test)]
mod image_judgment_tests;
mod judgment;
#[cfg(test)]
mod judgment_tests;
mod moment;
mod pixel_check;
mod region;
mod report;
mod run;
mod stats;
mod sun_disk_diff;
mod sun_disk_invariance;
mod terrain_band;

use std::path::PathBuf;
use std::process::ExitCode;

use error::時刻帯ごとの空の検収エラー;

const 出力ディレクトリ: &str = "target/sky_time";

pub fn 時刻別の空を確認する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] sky-time成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] sky-time失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, 時刻帯ごとの空の検収エラー> {
    if !crate::gen_source_assets::検証用ソースアセットを生成して成否を返す() || !crate::compile_assets::地形世界を既定で生成する()
    {
        return Err(時刻帯ごとの空の検収エラー::検証用アセットを生成できなかった);
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    let 実行環境 = run::実行環境を作る(出力先.clone())?;

    let マスク = draw::領域マスクを撮る(&実行環境)?;
    let 幅 = マスク.幅().画素数();
    let 区分 = region::領域区分::作る(&マスク)?;
    let 絵 = draw::絵を撮る(&実行環境)?;
    let mut 実測一覧 = Vec::new();
    for 時刻の絵 in &絵.時刻ごと {
        実測一覧.push(stats::採る(時刻の絵, &区分)?);
    }
    let 名前一覧: Vec<&str> = moment::代表時刻一覧.iter().map(|時刻| 時刻.名前).collect();
    report::表を出す(&名前一覧, &実測一覧);

    image_judgment::決定性を判定する(&絵)?;
    judgment::時刻間の変化を判定する(&名前一覧, &実測一覧, 区分.幾何画素数)?;
    judgment::夜明けと夕方の対称性を判定する(&実測一覧)?;
    judgment::夜を判定する(&実測一覧[moment::夜の添字])?;
    judgment::空の飽和を判定する(&名前一覧, &実測一覧)?;
    let 地形の帯 = terrain_band::破綻防止帯を判定する(&名前一覧, &実測一覧, 区分.幾何画素数)?;
    let 円盤の差分画素数 = image_judgment::太陽円盤を判定する(絵.円盤.ポストあり(), 絵.円盤.ポストあり対照())?;
    let 円盤の不変 = sun_disk_invariance::円盤が動かさないものを判定する(&絵.円盤)?;
    pixel_check::照合する(&実行環境, 幅, &区分)?;

    let 絵の置き場 = report::絵を書き出す(&出力先)?;
    Ok(format!(
        "空画素{}・幾何画素{}を4時刻で数え、隣り合う時刻の空色と明部と暗部がすべて変わり、夜明けと夕方の空平均色はバイト一致し、夜は間接照明だけの明るさで黒つぶれ0、どの時刻も空は飽和せず、夕方の太陽円盤は対照との差分{円盤の差分画素数}画素、{円盤の不変}、{地形の帯}、天頂寄りと地平線直上の代表画素がCPU正本と許容内で一致し、絵は{絵の置き場}",
        区分.空画素数, 区分.幾何画素数
    ))
}
