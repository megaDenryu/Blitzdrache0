//! 着手順5の第5段階(点光源の影)の検収入口。屋内の多光源の世界と夜の多光源の世界を本番の描画経路で走らせ、
//! 影が壁を遮ることと、影付きの灯2件が別々の立方体を読んでいることを画素の領域で判定し、計器を表にする。
//!
//! 判定は4つである。屋内の6領域が影の有無と灯の有無で宣言どおりに動くこと、夜の4領域が灯ごとの影の形どおりに動くこと、
//! どの実行もvalidationの指摘が0件であること、点光源の影の計器が件数から導いた値と一致することである。
//! **GPU時間には判定を置かない。** 枠の制定は実測を読んでから人が行う
//! (参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断h」)。
//!
//! 絵の合否も判定に含まれない。書き出したPNGを見て決めるのは親エージェントである。
//! 参照: `_doc/設計/クラスタ多光源と点光源の影.md`「第5段階の検収の入口」

mod instrument;
mod instrument_judgment;
mod interior_judgment;
mod interior_measure;
mod interior_region;
mod night_judgment;
mod night_measure;
mod night_region;
mod region;
mod shadow_count_scale;
mod summary;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::multi_light_world::world;

const 出力ディレクトリ: &str = "target/point_light_shadow";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] point-light-shadow成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] point-light-shadow失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    world::屋内の世界を用意する()?;
    world::夜の世界を用意する()?;
    crate::release_build::計測用に構築する("point-light-shadow")?;
    let 屋内の実行環境 = world::屋内の世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;
    let 夜の実行環境 = world::夜の世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;

    let 屋内 = interior_measure::屋内を3条件で撮る(&屋内の実行環境)?;
    summary::屋内の領域の表を表示する(&屋内.領域一覧);
    interior_judgment::屋内の領域を判定する(&屋内.領域一覧)?;

    let 夜 = night_measure::夜の世界を影付きの件数3つで撮る(&夜の実行環境)?;
    summary::夜の領域の表を表示する(&夜.領域一覧);
    night_judgment::夜の領域を判定する(&夜.領域一覧)?;
    instrument_judgment::点光源の影の計器を判定する(&夜.影付き2件の計器)?;
    instrument_judgment::夜の世界の絞りが落としたかを判定する(&夜.影付き2件の計器)?;

    let 計器一覧 = shadow_count_scale::影付きの件数を振って点光源の影の計器を採る(&屋内の実行環境)?;
    summary::点光源の影の計器の表を表示する(&計器一覧);
    for 計器 in &計器一覧 {
        instrument_judgment::点光源の影の計器を判定する(計器)?;
    }
    Ok(summary::検収の要約を組む(
        &夜.影付き2件の計器,
        &[&屋内.影付きの絵, &屋内.影なしの絵, &夜.絵],
    ))
}
