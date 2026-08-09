//! 乱数の種からクソゲー1本目のマップを作る入口。担当するのは種の綴りの受け取りと、ソースアセットの生成から
//! 実行時形式へ焼くまでの2工程を順に呼ぶことだけである。生成の中身はアセットコンパイラのexampleが持つ。
//!
//! 種を必須にするのは、既定の種を置くとその1つが暗黙の正本になり、種を変えた生成物と既定の生成物が
//! 同じ出力ルートで見分けられなくなるためである。
//! 参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断7: 地図の正本を持ち、生成は2系統に分ける」

use std::process::{Command, ExitCode};

use crate::compile_assets;

/// 種を導く選択肢の綴り。この綴りの直後の1語が種の値である。
const 種の選択肢の綴り: &str = "--seed";

/// アセットコンパイラのexampleへ種を渡すときの綴り。
/// 綴りは`crates/blitz_asset_compiler/examples/generate_source_assets/map_seed.rs`にも同じものがあり、
/// 食い違えば生成器が「知らない引数である」で失敗する。
const 生成器へ渡す綴り: &str = "--game-map-seed";

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let 種 = match 引数一覧から種を読む(引数一覧) {
        Ok(種) => 種,
        Err(理由) => {
            eprintln!("[xtask] gen-game-map失敗: {理由}");
            eprintln!("使い方: cargo xtask gen-game-map --seed <32ビットの非負整数>");
            return ExitCode::FAILURE;
        }
    };
    if 種からマップを生成する(&種) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// ソースアセットを種から書き出し、続けて実行時形式へ焼く。焼く工程は既存のコンパイル入口をそのまま呼ぶ。
pub fn 種からマップを生成する(種: &str) -> bool {
    ソースアセットを種から書き出す(種) && compile_assets::場所巡り世界を既定で生成する()
}

fn 引数一覧から種を読む(引数一覧: &[String]) -> Result<String, String> {
    let [綴り, 値] = 引数一覧 else {
        return Err(format!("引数は{種の選択肢の綴り}と種の値の2語である"));
    };
    if 綴り != 種の選択肢の綴り {
        return Err(format!("知らない引数である: {綴り}"));
    }
    値.parse::<u32>()
        .map(|種| 種.to_string())
        .map_err(|誤り| format!("種を32ビットの非負整数として読めない({値}): {誤り}"))
}

fn ソースアセットを種から書き出す(種: &str) -> bool {
    println!("[xtask] 場所巡りの世界のソースアセットを種{種}から生成");
    let 起動結果 = Command::new("cargo")
        .args(["run", "-p", "blitz_asset_compiler", "--example", "generate_source_assets", "--"])
        .args([生成器へ渡す綴り, 種])
        .status();
    match 起動結果 {
        Ok(終了状態) if 終了状態.success() => true,
        Ok(終了状態) => {
            eprintln!("[xtask] 場所巡りの世界のソースアセット生成が終了コード{終了状態}で失敗");
            false
        }
        Err(起動誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {起動誤り}");
            false
        }
    }
}
