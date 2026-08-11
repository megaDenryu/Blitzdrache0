//! 時間再構成の絵をオーナーが目で確かめるための入口。目視見本の庭を、時間再構成を効かせた側と切った側の同じ構図で
//! 撮り、輪郭の一区画を拡大した対も併せて書き出す。
//!
//! 画素の基準値を持たないのは、この入口の目的が「輪郭の階段が減ったか、絵が鈍っていないか」の判断材料を出すことで
//! あり、その判断は絵を見る人が行うためである。機械が見るのはvalidationの指摘が0件であることと、効かせた側と
//! 切った側の絵が実際に食い違うことの2つだけである。後者を見るのは、起動指定の取り違えで同じ絵を2枚並べたまま
//! 「差が無い」と読む事故を防ぐためである。
//! 参照: `_doc/設計/時間再構成.md`「段割りと各段の完了条件」の段4

mod crop;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::acceptance::{検収の実行名, 読み戻しの書き出し先};
use crate::day_moment::{代表時刻, 代表時刻一覧};
use run::時間再構成の条件;

const 出力ディレクトリ: &str = "target/temporal_visual";

/// 撮る時刻のダンプ名。夜明けは低い太陽で影が長く伸びる時刻であり、正午は太陽が高く輪郭の明暗差が最も大きい時刻である。
/// 4時刻すべてを撮らないのは、目視の対がその都度2枚に増え、見る側の負担が増えるためである。
const 撮る時刻のファイル名: [&str; 2] = ["dawn", "noon"];

pub fn 実行する() -> ExitCode {
    match 撮る() {
        Ok(要約) => {
            println!("[xtask] temporal-visual成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] temporal-visual失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 撮る() -> Result<String, String> {
    crate::visual_sample_world::用意する()?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let mut 行一覧 = Vec::new();
    for 時刻 in 代表時刻一覧.iter().filter(|時刻| 撮る時刻のファイル名.contains(&時刻.ファイル名)) {
        行一覧.push(一時刻の対を撮る(&出力先, 時刻)?);
    }
    if 行一覧.len() != 撮る時刻のファイル名.len() {
        return Err(format!("代表時刻の表から{}時刻しか選べなかった", 行一覧.len()));
    }
    Ok(行一覧.join("、"))
}

fn 一時刻の対を撮る(出力先: &std::path::Path, 時刻: &代表時刻) -> Result<String, String> {
    let 効かせた = 一条件を撮る(出力先, 時刻, 時間再構成の条件::効かせる, "with_taa")?;
    let 切った = 一条件を撮る(出力先, 時刻, 時間再構成の条件::切る, "without_taa")?;
    crop::区画が絵に収まるか確かめる(効かせた.画像.幅().画素数(), 効かせた.画像.高さ().画素数())?;
    let 食い違い = 食い違う画素を数える(効かせた.画像.バイト列(), 切った.画像.バイト列())?;
    if 食い違い == 0 {
        return Err(format!("{}の効かせた絵と切った絵が1画素も食い違わない", 時刻.名前));
    }
    let 効かせた拡大 = crop::切り取って拡大する(&効かせた.png, &出力先.join(format!("{}_with_taa_crop.png", 時刻.ファイル名)))?;
    let 切った拡大 = crop::切り取って拡大する(&切った.png, &出力先.join(format!("{}_without_taa_crop.png", 時刻.ファイル名)))?;
    Ok(format!(
        "{}は全{}画素中{}画素が食い違い(効かせた絵{}、切った絵{}、拡大は{}と{})",
        時刻.名前,
        効かせた.画像.画素数(),
        食い違い,
        効かせた.png.display(),
        切った.png.display(),
        効かせた拡大.display(),
        切った拡大.display()
    ))
}

struct 撮った絵 {
    画像: crate::acceptance::読み戻し画像,
    png: PathBuf,
}

fn 一条件を撮る(
    出力先: &std::path::Path, 時刻: &代表時刻, 条件: 時間再構成の条件, 接尾辞: &str
) -> Result<撮った絵, String> {
    let 名前 = format!("{}_{}", 時刻.ファイル名, 接尾辞);
    let 実行名 = 検収の実行名::生成する(&名前)?;
    let 書き出し先 = 読み戻しの書き出し先::出力ディレクトリの中に決める(出力先, 実行名);
    let 画像 = run::描画する(&書き出し先, 時刻.一日内秒, 条件)?;
    let png = 書き出し先.目視用の絵へ変換する()?;
    Ok(撮った絵 { 画像, png })
}

fn 食い違う画素を数える(左: &[u8], 右: &[u8]) -> Result<usize, String> {
    if 左.len() != 右.len() {
        return Err(format!("比べる2枚のバイト数が違う: {}と{}", 左.len(), 右.len()));
    }
    Ok(左
        .chunks_exact(4)
        .zip(右.chunks_exact(4))
        .filter(|(左画素, 右画素)| 左画素 != 右画素)
        .count())
}
