//! `bake-building`コマンド: 建物定義を1件名指し、その1棟だけを平らな地面の中央へ据えた検証世界を
//! ソースから焼いて歩行の器で開く。編集サーバーを起こさずに、建物エディターが保存した格子の姿を確かめる入口である。
//!
//! 3つの工程(編集サーバーのビンによるソースの書き出し・`compile-assets`と同じ焼き付け・歩行の器の起動)を
//! ここが繋ぐ。実行時アセットの置き場の綴りをここが持つのは、世界ごとの`target/`の下の置き場を
//! `compile_assets/default_root.rs`が持つのと同じ理由であり、焼く側と開く側が同じ場所を見るためである。
//!
//! 参照: `_doc/計画/エディターからゲームまでの統合の作戦.md`「段G」

mod launch;
mod source_export;

use std::path::Path;
use std::process::ExitCode;

use crate::asset_generator::世界名;
use crate::compile_assets;
use crate::editor::building_outline_catalog;
use crate::editor::project_root::プロジェクトルート;

/// 建物1棟だけの検証世界の実行時アセットの置き場。焼く側と開く側が同じ綴りを見る。
const 検証世界の出力ルート: &str = "target/one_building_assets";

/// 焼きまでで止めて窓を開かない選択肢。機械の検収がこれを付けて呼ぶ。
const 焼きまでで止める選択肢: &str = "--bake-only";

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    match 焼いて開く(引数一覧) {
        Ok(コード) => コード,
        Err(誤り) => {
            eprintln!("[xtask] {誤り}");
            ExitCode::FAILURE
        }
    }
}

fn 焼いて開く(引数一覧: &[String]) -> Result<ExitCode, String> {
    let 建物定義の識別子 = 建物定義の識別子を引数から取り出す(引数一覧)?;
    let リポジトリルート = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let プロジェクトルート = プロジェクトルート::引数から解く(引数一覧, &リポジトリルート);

    let カタログ = building_outline_catalog::既定のファイルへ書き出す(&リポジトリルート, &プロジェクトルート)?;
    println!("建物外形カタログ: {}", カタログ.display());

    source_export::編集サーバーのビンで書き出す(&リポジトリルート, 建物定義の識別子, 引数一覧)?;
    let 焼けたか = compile_assets::生成する(
        compile_assets::ソースルート(),
        Path::new(検証世界の出力ルート),
        世界名::建物一棟の検証世界,
    );
    if !焼けたか {
        return Err(format!("建物{建物定義の識別子}の検証世界を焼けなかった"));
    }
    if 引数一覧.iter().any(|引数| 引数 == 焼きまでで止める選択肢) {
        println!("[xtask] {焼きまでで止める選択肢}が付いているため、窓を開かずに終える: {検証世界の出力ルート}");
        return Ok(ExitCode::SUCCESS);
    }
    Ok(launch::歩行の器で開く(検証世界の出力ルート))
}

/// 建物定義の識別子は先頭の引数だけから読む。選択肢でない引数を探し回らないのは、`--project <ルート>`の
/// 値のような「選択肢に続く値」を建物定義の識別子と取り違えないためである。既定へ倒さないのは、
/// 名指しの無い実行が意図しない建物を焼くことを防ぐためである。
fn 建物定義の識別子を引数から取り出す(引数一覧: &[String]) -> Result<&str, String> {
    let 使い方 = "使い方: cargo xtask bake-building <建物定義ID> [--project <ルート>] [--bake-only]";
    match 引数一覧.first() {
        Some(先頭) if !先頭.starts_with("--") => Ok(先頭.as_str()),
        _ => Err(format!("建物定義の識別子を先頭の引数として渡していない。{使い方}")),
    }
}
