//! 起動バイナリ 兼 コンポジションルート。
//!
//! 責務: ウィンドウ生成(winit)と各層の依存の配線だけを行う。
//! ゲームロジック・エンジン処理をここに書いてはならない。

#![forbid(unsafe_code)]

mod app;
mod atmosphere_medium;
#[cfg(test)]
mod atmosphere_medium_tests;
mod cli;
mod dev_ui;
mod embedded_cloth_shaders;
mod embedded_derived_environment_shaders;
mod embedded_distant_environment_shader;
mod embedded_shaders;
mod embedded_sky_shaders;
mod error;
mod hot_reload;
mod input;
mod reports;
mod smoke;

use std::process::ExitCode;

use blitz_render::クリアカラー;
use winit::event_loop::{ControlFlow, EventLoop};

use app::アプリ;
use error::起動エラー;

fn main() -> ExitCode {
    match 実行する() {
        Ok(終了コード) => 終了コード,
        Err(誤り) => {
            eprintln!("起動に失敗した: {誤り}");
            ExitCode::FAILURE
        }
    }
}

fn 実行する() -> Result<ExitCode, 起動エラー> {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    match cli::引数を解析する(&引数一覧)? {
        cli::起動要求::天空状態報告 => Ok(reports::sky_state::天空状態表を出す()),
        cli::起動要求::大気のベイク済み画像報告 => Ok(reports::atmosphere_lut::大気のベイク済み画像表を出す()),
        cli::起動要求::遠方環境報告 => Ok(reports::distant_environment::遠方環境表を出す()),
        cli::起動要求::描画実行(起動設定) => 描画する(*起動設定),
    }
}

fn 描画する(起動設定: cli::起動設定) -> Result<ExitCode, 起動エラー> {
    // M0のクリアカラー: 落ち着いた濃紺。
    let クリア色 = クリアカラー::生成する(0.05, 0.07, 0.12, 1.0)?;

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut アプリ = アプリ::生成する(起動設定, クリア色)?;
    event_loop.run_app(&mut アプリ)?;

    if let Some(誤り) = アプリ.起動時エラーを取り出す() {
        return Err(誤り);
    }

    終了処理する(アプリ)
}

fn 終了処理する(mut アプリ: アプリ) -> Result<ExitCode, 起動エラー> {
    let 検証カウンタ = アプリ.検証カウンタを取得する();
    // 判断30: レンダラー内部(クエリプール・移動平均・作業領域)を読む報告は破棄より前に出す
    // (検証カウンタとは逆に、破棄後では読めない)。
    reports::exit::終了時報告を出す(&アプリ);
    // 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断3」。
    // 読み取りは必ずレンダラー破棄後に行う。
    アプリ.レンダラーを破棄する();

    let Some(検証カウンタ) = 検証カウンタ else {
        return Ok(ExitCode::SUCCESS);
    };
    let 件数 = 検証カウンタ.件数を読む();
    println!("validationエラー・警告合計件数: {件数}");
    if 件数 > 0 { Ok(ExitCode::FAILURE) } else { Ok(ExitCode::SUCCESS) }
}
