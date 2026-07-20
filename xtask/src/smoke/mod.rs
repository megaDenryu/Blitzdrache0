//! M3のDoD自動検証: shaders/scene.slangとassets/smoke/を一時コピーへ複製し、
//! quadステージ(600フレーム、リサイズ・最小化復帰・アセット/シェーダーホットリロード
//! を機械検証)→ (DamagedHelmet取得済みなら)helmetステージ(120フレーム)を順に実行する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断22」。

mod copy_setup;
mod run_stage;

use std::path::Path;
use std::process::ExitCode;

const 四角形フレーム数: &str = "600";
const ヘルメットフレーム数: &str = "120";
const ヘルメット取得先: &str = "assets/samples/DamagedHelmet/DamagedHelmet.glb";

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
    if !run_stage::実行する(四角形フレーム数, &シェーダーコピー先, Some(&アセットルート), "quad") {
        eprintln!("[xtask] smoke失敗: quadステージ");
        return ExitCode::FAILURE;
    }
    println!("[xtask] quadステージ成功");

    if Path::new(ヘルメット取得先).is_file() {
        println!("[xtask] helmetステージ実行");
        if !run_stage::実行する(ヘルメットフレーム数, &シェーダーコピー先, None, "helmet") {
            eprintln!("[xtask] smoke失敗: helmetステージ");
            return ExitCode::FAILURE;
        }
        println!("[xtask] helmetステージ成功");
    } else {
        println!(
            "[xtask] helmetアセット未取得のためhelmetステージをスキップした(cargo xtask fetch-assetsで取得可)"
        );
    }

    println!("[xtask] smoke成功: validation・ピクセル判定・ホットリロードすべて成功で終了した");
    ExitCode::SUCCESS
}
