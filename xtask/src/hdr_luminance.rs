//! 順3-IIb(自動露出)のヒストグラムの輝度範囲を実測から制定するための入口。
//! 担当するのは、目視見本世界を一日の代表4時刻で起動し、明るさの圧縮を通す前のHDRカラーを読み戻して、その画面全体のlog2輝度の分布を報告することである。
//!
//! 圧縮前を読むのは、明るさの圧縮とsRGB符号化を通った後の8ビット値から圧縮前の値を復元できないためである。
//! 画面全体で採るのは、自動露出が画面全体のヒストグラムを見るためであり、空も太陽円盤も除かない。
//!
//! 画素の基準値は持たない。範囲の制定はここで採った値を人が読んで決めることであり、期待値をこの入口に置くと、まだ決めていない範囲を検収側が先に決めたことになる。
//! 機械判定は2つだけである。validationの指摘が0件であることと、報告する統計値がすべて有限であることである。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「順3-IIの実装設計」

mod error;
mod judgment;
mod record;
mod run;
mod statistics;
mod table;

use std::path::PathBuf;
use std::process::ExitCode;

use error::圧縮前の輝度分布の計測エラー;

use crate::acceptance::検収の実行名;
use crate::release_build::計測の生値のファイル;

use crate::day_moment::代表時刻一覧;

const 出力ディレクトリ: &str = "target/hdr_luminance";

pub(crate) fn hdr輝度を実測する() -> ExitCode {
    match 計測する() {
        Ok(要約) => {
            println!("[xtask] hdr-luminance成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] hdr-luminance失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 計測する() -> Result<String, 圧縮前の輝度分布の計測エラー> {
    crate::visual_sample_world::用意する().map_err(圧縮前の輝度分布の計測エラー::目視見本世界を用意できなかった)?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    let 実行環境 = run::実行環境を作る(出力先.clone())?;

    let mut 標本一覧 = Vec::with_capacity(代表時刻一覧.len());
    for 時刻 in &代表時刻一覧 {
        println!("[xtask] hdr-luminance描画: {}", 時刻.名前);
        let 画像 = 実行環境
            .圧縮前の画素で描いて読み戻す(検収の実行名::生成する(時刻.ファイル名)?, &run::時刻の起動指定を組み立てる(時刻.一日内秒))?;
        let 統計 = statistics::集計する(&画像)?;
        judgment::値が有限であることを確かめる(&統計, 時刻.名前)?;
        標本一覧.push((時刻.名前, 統計));
    }
    record::生値を書く(&計測の生値のファイル::出力ディレクトリの中の場所(&出力先), &標本一覧)?;
    table::表示する(&標本一覧);
    Ok(要約を組む(&標本一覧, &出力先))
}

fn 要約を組む(標本一覧: &[(&'static str, statistics::輝度統計)], 出力先: &std::path::Path) -> String {
    let 行一覧: Vec<String> = 標本一覧.iter().map(|(名前, 統計)| table::要約の行(名前, 統計)).collect();
    format!("{}。時刻別の表と生値は{}にある", 行一覧.join("、"), 出力先.display())
}
