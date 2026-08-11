//! 乱数の種からクソゲー1本目のマップを作る入口。担当するのは種の綴りの受け取りと、ソースアセットの生成から
//! 実行時形式へ焼くまでの2工程を順に呼ぶことだけである。生成の中身はアセットコンパイラのexampleが持つ。
//!
//! 種を必須にするのは、既定の種を置くとその1つが暗黙の正本になり、種を変えた生成物と既定の生成物が
//! 同じ出力ルートで見分けられなくなるためである。生成器の起動そのものは`source_write`が持つ。
//! 参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断7: 地図の正本を持ち、生成は2系統に分ける」

pub mod source_write;

use std::process::ExitCode;

use crate::compile_assets;

/// 種を導く選択肢の綴り。この綴りの直後の1語が種の値である。
const 種の選択肢の綴り: &str = "--seed";

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
    match source_write::既定のソースルートへ書き出す(種) {
        Ok(_) => compile_assets::場所巡り世界を既定で生成する(),
        Err(理由) => {
            eprintln!("[xtask] {理由}");
            false
        }
    }
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
