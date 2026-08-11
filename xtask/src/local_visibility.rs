//! 順3-IIa-3の検収入口。CPU正本が焼いた合成深度をGPUの深度画像へ注入し、本番のフレーム経路で走った遮蔽の
//! 標本化と両側ぼかしの結果を読み戻して、同じ正本と突き合わせる。
//!
//! 4つの形を並べるのは、局所可視度の期待される振る舞いが形そのものから言えるためである。平面は遮蔽が起きない形、
//! 段差は片側だけが暗くなる形、球は凸で偽の遮蔽が出ない形、凹面は法線が連続に変わりながら遮蔽が生まれる形であり、
//! 実物のシーンを撮った深度からはこの4つの性質を分けて読めない。
//!
//! 突き合わせそのものはblitz_appが行い、この入口は報告の行を読んで判定する。CPU正本を読めるのが
//! コンポジションルートだけであり、正本の写しを入口がもう1つ持つと、その写しの正しさが検収の前提になるためである。
//!
//! 局所可視度を0と中間と1へ固定した3枚の対を別に採るのは、掛かる先が拡散間接だけであることを画素の値で
//! 反証するためである。深度から焼いた可視度は画素ごとに値が違い、この比較が成立しない。
//!
//! 3枚は別々のプロセスで撮るため、同じ構図で撮れていることを数を採る前に確かめる。アプリはプロセスをまたいだ
//! 絵の一致を保証していない(`composition_guard`に観測と根拠がある)。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」

mod composition_guard;
mod judgment;
mod parse;
mod run;
mod shape_property;
mod summary;
mod term_judgment;
mod term_measure;
mod term_run;
mod term_tally;
mod tolerance;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::acceptance::{圧縮前のHDR画像, 描画検収の実行環境, 検収の実行名, 検収エラー};

const 出力ディレクトリ: &str = "target/local_visibility";

/// 起動指定へ渡す形の語。並びは平面・段差・球・凹面であり、遮蔽が出ない形から出る形へ向かう。
const 形の一覧: [&str; 4] = ["plane", "step", "sphere", "concave"];

/// 局所可視度を固定する符号値。0は全遮蔽、128は中間、255は遮蔽なしである。
const 全遮蔽の符号値: u8 = 0;
const 中間の符号値: u8 = 128;
const 遮蔽なしの符号値: u8 = 255;

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] local-visibility成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] local-visibility失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    crate::visual_sample_world::用意する().map_err(|破れ| 破れ.to_string())?;
    if !crate::compile_assets::既定を生成する() {
        return Err("間接照明の検収世界のアセット生成に失敗した".to_string());
    }
    let 形の実行環境 = run::実行環境を作る();
    let 項別の実行環境 = term_run::実行環境を作る(PathBuf::from(出力ディレクトリ))?;

    let mut 行一覧 = Vec::new();
    for 形 in 形の一覧 {
        let 報告 = 形の実行環境.報告を採る(検収の実行名::生成する(形)?, &run::形の起動指定を組み立てる(形))?;
        let 行 = parse::形の行を取り出す(&報告)?;
        judgment::共通の判定を行う(&行)?;
        shape_property::形の性質を検査する(&行)?;
        行一覧.push(行);
    }

    let 遮蔽なし = 一つの符号値で描く(&項別の実行環境, 遮蔽なしの符号値)?;
    let 中間 = 一つの符号値で描く(&項別の実行環境, 中間の符号値)?;
    let 全遮蔽 = 一つの符号値で描く(&項別の実行環境, 全遮蔽の符号値)?;
    composition_guard::構図の一致を確かめる(&遮蔽なし, &中間, &全遮蔽)?;
    let 項別 = term_measure::測る(&遮蔽なし, &中間, &全遮蔽, 中間の符号値)?;
    term_judgment::判定する(&項別)?;

    Ok(summary::要約を組む(&行一覧, &項別))
}

/// 局所可視度を1つの符号値へ固定して1条件を撮る。
fn 一つの符号値で描く(実行環境: &描画検収の実行環境, 符号値: u8) -> Result<圧縮前のHDR画像, 検収エラー> {
    実行環境.圧縮前の画素で描いて読み戻す(term_run::実行名を組む(符号値)?, &term_run::符号値の起動指定を組み立てる(符号値))
}
