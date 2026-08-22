//! エディター関連コマンドの割り当て。command_catalogの`editor`分類と同じ範囲
//! (エディターサーバーの起動・建物外形カタログの書き出し・建物1棟だけの焼き・型契約の書き出しの4件)を担当する。

use std::process::ExitCode;

use crate::bake_building;
use crate::contract_export;
use crate::editor::{self, building_outline_catalog};

pub(super) fn 割り当てる(名前: &str, 引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "editor" => Some(実行結果をコードへ変換する(editor::実行する(引数一覧))),
        "building-outline-catalog" => Some(実行結果をコードへ変換する(building_outline_catalog::実行する(引数一覧))),
        "bake-building" => Some(bake_building::実行する(引数一覧)),
        "contract-export" => Some(実行結果をコードへ変換する(contract_export::実行する())),
        _ => None,
    }
}

fn 実行結果をコードへ変換する(結果: Result<(), String>) -> ExitCode {
    match 結果 {
        Ok(()) => ExitCode::SUCCESS,
        Err(誤り) => {
            eprintln!("[xtask] {誤り}");
            ExitCode::FAILURE
        }
    }
}
