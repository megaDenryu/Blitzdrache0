//! DoD自動検証: shaders/とassets/smoke/を一時コピーへ複製し、quad(ホットリロード+厳密判定)→helmet→particles→dev-ui→shadow(輝度相対比較)→fox(アニメ差分)→cloth(布込み差分)→post-resize(ポスト有効のままウィンドウ再構築)の各ステージを順に実行する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断22」「判断29」「判断37」「判断45」「判断55」。

mod copy_setup;
mod launch_setting;
mod run_stage;
mod stage_catalog;
mod stages;

use std::process::ExitCode;

use crate::verify::{検証の出力の置き場名, 検証の出力ルート};

/// スモークが読む実行時形式の置き場。一時コピーしたソースから焼く先であり、既定の置き場を汚さない。
const スモークの実行時形式の置き場: 検証の出力の置き場名 = 検証の出力の置き場名::生成する("smoke_runtime_assets");

pub fn スモークを実行する() -> ExitCode {
    if !crate::gen_source_assets::生成物のソースアセットを揃えて成否を返す() {
        return ExitCode::FAILURE;
    }
    let シェーダーコピー先 = match copy_setup::シェーダーを一時コピーする() {
        Ok(パス) => パス,
        Err(誤り) => {
            eprintln!("[xtask] シェーダーの一時コピーに失敗した: {誤り}");
            return ExitCode::FAILURE;
        }
    };
    let アセットルート = match copy_setup::アセットを一時コピーする() {
        Ok(パス) => パス,
        Err(誤り) => {
            eprintln!("[xtask] アセットの一時コピーに失敗した: {誤り}");
            return ExitCode::FAILURE;
        }
    };
    let 実行時アセットルート = 検証の出力ルート::既定().名前が指す置き場(スモークの実行時形式の置き場);
    if !crate::compile_assets::既定を生成する()
        || !crate::compile_assets::実行時形式を生成する(&アセットルート, &実行時アセットルート, crate::asset_generator::世界名::板の世界)
    {
        return ExitCode::FAILURE;
    }

    if !stages::すべてを実行する(&シェーダーコピー先, &実行時アセットルート) {
        return ExitCode::FAILURE;
    }

    println!("[xtask] smoke成功: validation・ピクセル判定・ホットリロードすべて成功で終了した");
    ExitCode::SUCCESS
}
