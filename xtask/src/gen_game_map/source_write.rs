//! ソースアセットの生成器をプロセスとして起動する工程。受け取るのは種と、指定するならソースルート、
//! 返すのは取り込んだ標準出力である。
//!
//! 選択肢の綴りをここへ集めるのは、生成器側(`crates/blitz_asset_compiler/examples/generate_source_assets/generation_arguments.rs`)
//! にも同じ綴りがあり、食い違えば生成器が「知らない引数である」で失敗するためである。
//!
//! 標準出力を取り込むのは、増分が効いたかを外から見る唯一の材料が生成器の報告の行だからである。
//! 取り込んだ内容はそのまま表示するため、呼び出し側の画面に出るものは流したときと変わらない。

use std::path::Path;
use std::process::Command;

const 生成器へ渡す種の綴り: &str = "--game-map-seed";
const 生成器へ渡すソースルートの綴り: &str = "--source-root";

pub fn 既定のソースルートへ書き出す(種: &str) -> Result<String, String> {
    起動して標準出力を取り込む(種, None)
}

pub fn ソースルートを指定して書き出す(種: &str, ソースルート: &Path) -> Result<String, String> {
    起動して標準出力を取り込む(種, Some(ソースルート))
}

fn 起動して標準出力を取り込む(種: &str, ソースルート: Option<&Path>) -> Result<String, String> {
    println!("[xtask] 場所巡りの世界のソースアセットを種{種}から生成");
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_asset_compiler", "--example", "generate_source_assets", "--"])
        .args([生成器へ渡す種の綴り, 種]);
    if let Some(ルート) = ソースルート {
        コマンド.arg(生成器へ渡すソースルートの綴り).arg(ルート);
    }
    let 出力 = コマンド.output().map_err(|起動誤り| format!("cargoの起動に失敗: {起動誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    print!("{標準出力}");
    eprint!("{}", String::from_utf8_lossy(&出力.stderr));
    if 出力.status.success() {
        Ok(標準出力)
    } else {
        Err(format!("場所巡りの世界のソースアセット生成が終了コード{}で失敗", 出力.status))
    }
}
