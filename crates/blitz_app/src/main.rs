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
mod embedded_auto_exposure_shaders;
mod embedded_cloth_shaders;
mod embedded_cluster_light_assignment_shader;
mod embedded_derived_environment_shaders;
mod embedded_distant_environment_shader;
mod embedded_local_visibility_shaders;
mod embedded_shaders;
mod embedded_sky_shaders;
mod embedded_temporal_reconstruction_shaders;
mod error;
mod game;
mod hot_reload;
mod input;
mod overlay_ui;
mod reports;
mod runtime_assets;
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
        cli::起動要求::派生表現報告 => Ok(reports::derived_environment::派生表現表を出す()),
        cli::起動要求::太陽天頂区間の跨ぎ報告 => Ok(reports::sun_zenith_crossing::太陽天頂区間の跨ぎ表を出す()),
        cli::起動要求::描画実行(起動設定) => 描画する(*起動設定),
    }
}

fn 描画する(起動設定: cli::起動設定) -> Result<ExitCode, 起動エラー> {
    // M0のクリアカラー: 落ち着いた濃紺。検収の探り色が指定された実行だけ、背景をその線形RGBで埋める。
    let クリア色 = match 起動設定.読み戻し検収.自動露出の探り色 {
        Some([赤, 緑, 青]) => クリアカラー::生成する(赤, 緑, 青, 1.0)?,
        None => クリアカラー::生成する(0.05, 0.07, 0.12, 1.0)?,
    };

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut アプリ = アプリ::生成する(起動設定, クリア色)?;
    event_loop.run_app(&mut アプリ)?;

    if let Some(誤り) = アプリ.起動時エラーを取り出す() {
        return Err(誤り);
    }

    終了処理する(アプリ)
}

/// 検収がvalidationの件数を読むための見出し。綴りは`xtask/src/validation_count.rs`の写しと一致していなければならず、
/// 食い違いは`cargo xtask conform`の綴りの契約の検査が拒む。
const 検証層の指摘件数の見出し: &str = "validationエラー・警告合計件数:";

fn 終了処理する(mut アプリ: アプリ) -> Result<ExitCode, 起動エラー> {
    let 検証カウンタ = アプリ.検証カウンタを取得する();
    // 判断30: レンダラー内部(クエリプール・移動平均・作業領域)を読む報告は破棄より前に出す
    // (検証カウンタとは逆に、破棄後では読めない)。
    アプリ.終了時報告を出す();
    // 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断3」。
    // 読み取りは必ずレンダラー破棄後に行う。
    アプリ.レンダラーを破棄する();

    let Some(検証カウンタ) = 検証カウンタ else {
        return Ok(ExitCode::SUCCESS);
    };
    let 件数 = 検証カウンタ.件数を読む();
    println!("{検証層の指摘件数の見出し} {件数}");
    if 件数 > 0 { Ok(ExitCode::FAILURE) } else { Ok(ExitCode::SUCCESS) }
}
