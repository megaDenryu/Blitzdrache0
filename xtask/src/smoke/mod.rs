//! DoD自動検証: shaders/とassets/smoke/を一時コピーへ複製し、quad(ホットリロード+厳密判定)→helmet→particles→dev-ui→shadow(輝度相対比較)→fox(アニメ差分)→cloth(布込み差分)の各ステージを順に実行する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断22」「判断29」「判断37」「判断45」「判断55」。

mod copy_setup;
mod run_stage;

use std::path::Path;
use std::process::ExitCode;

const 四角形フレーム数: &str = "600";
const ヘルメットフレーム数: &str = "120";
const 粒子フレーム数: &str = "120";
const 開発UIフレーム数: &str = "120";
const シャドウフレーム数: &str = "120";
const フォックスフレーム数: &str = "120";
const ヘルメット取得先: &str = "assets/samples/DamagedHelmet/DamagedHelmet.glb";
const フォックス取得先: &str = "assets/samples/Fox/Fox.glb";

pub fn 実行する() -> ExitCode {
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

    println!("[xtask] quadステージ実行");
    if !run_stage::実行する(
        四角形フレーム数,
        &シェーダーコピー先,
        Some(&アセットルート),
        "quad",
        true,
        false,
        false,
        true,
        false,
    ) {
        eprintln!("[xtask] smoke失敗: quadステージ");
        return ExitCode::FAILURE;
    }
    println!("[xtask] quadステージ成功");

    if Path::new(ヘルメット取得先).is_file() {
        println!("[xtask] helmetステージ実行");
        if !run_stage::実行する(
            ヘルメットフレーム数,
            &シェーダーコピー先,
            None,
            "helmet",
            false,
            false,
            false,
            false,
            false,
        ) {
            eprintln!("[xtask] smoke失敗: helmetステージ");
            return ExitCode::FAILURE;
        }
        println!("[xtask] helmetステージ成功");
    } else {
        println!("[xtask] helmetアセット未取得のためhelmetステージをスキップした(cargo xtask fetch-assetsで取得可)");
    }

    println!("[xtask] particlesステージ実行");
    if !run_stage::実行する(
        粒子フレーム数,
        &シェーダーコピー先,
        Some(&アセットルート),
        "quad",
        true,
        true,
        false,
        true,
        false,
    ) {
        eprintln!("[xtask] smoke失敗: particlesステージ");
        return ExitCode::FAILURE;
    }
    println!("[xtask] particlesステージ成功");

    println!("[xtask] dev-uiステージ実行");
    if !run_stage::実行する(
        開発UIフレーム数,
        &シェーダーコピー先,
        Some(&アセットルート),
        "quad",
        true,
        true,
        true,
        true,
        false,
    ) {
        eprintln!("[xtask] smoke失敗: dev-uiステージ");
        return ExitCode::FAILURE;
    }
    println!("[xtask] dev-uiステージ成功");

    println!("[xtask] shadowステージ実行");
    if !run_stage::実行する(
        シャドウフレーム数,
        &シェーダーコピー先,
        Some(&アセットルート),
        "shadow_scene",
        false,
        false,
        false,
        false,
        false,
    ) {
        eprintln!("[xtask] smoke失敗: shadowステージ");
        return ExitCode::FAILURE;
    }
    println!("[xtask] shadowステージ成功");

    if Path::new(フォックス取得先).is_file() {
        println!("[xtask] foxステージ実行");
        if !run_stage::実行する(フォックスフレーム数, &シェーダーコピー先, None, "fox", false, false, false, false, false) {
            eprintln!("[xtask] smoke失敗: foxステージ");
            return ExitCode::FAILURE;
        }
        println!("[xtask] foxステージ成功");
    } else {
        println!("[xtask] Foxアセット未取得のためfox/clothステージをスキップした(cargo xtask fetch-assetsで取得可)");
    }

    // clothステージ(判断55・56): quad+吊るし布でフレーム間差分判定(布シミュが動くことの実証。Fox不要)。
    println!("[xtask] clothステージ実行");
    if !run_stage::実行する(
        フォックスフレーム数,
        &シェーダーコピー先,
        Some(&アセットルート),
        "quad",
        true,
        false,
        false,
        false,
        true,
    ) {
        eprintln!("[xtask] smoke失敗: clothステージ");
        return ExitCode::FAILURE;
    }
    println!("[xtask] clothステージ成功");

    println!("[xtask] smoke成功: validation・ピクセル判定・ホットリロードすべて成功で終了した");
    ExitCode::SUCCESS
}
