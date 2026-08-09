//! アセットコンパイラのexampleをプロセスとして起動する工程。受け取るのはソースルートと出力ルートと世界名と、
//! 世界によっては要る同居植生の個体数とテクスチャ格納方針の名前、返すのは成否である。
//!
//! 起動そのものを親モジュールから分けるのは、引数の並べ方が選択肢の追加のたびに伸びるためであり、
//! 親が持つ「どの世界をどの出力ルートへ焼くか」の対応と押し合わせないためである。

use std::path::Path;
use std::process::Command;

/// コンパイラの入口が受け取る指定一式。方針の名前を渡さない呼び出しは、コンパイラ側の既定(`全てRGBA8`)で焼かれる。
pub(super) struct 実行時形式を焼く指定<'指定> {
    pub(super) ソースルート: &'指定 Path,
    pub(super) 出力ルート: &'指定 Path,
    pub(super) 世界名: &'指定 str,
    pub(super) 同居植生個体数: Option<&'指定 str>,
    pub(super) テクスチャ格納方針の名前: Option<&'指定 str>,
}

pub(super) fn アセットコンパイラを起動して実行時形式を焼く(指定: &実行時形式を焼く指定) -> bool {
    println!(
        "[xtask] 実行時アセット生成({}{}{}): {} -> {}",
        指定.世界名,
        指定.同居植生個体数.map_or_else(String::new, |個体数| format!(", 同居植生{個体数}体")),
        指定.テクスチャ格納方針の名前.map_or_else(String::new, |名前| format!(", 格納方針{名前}")),
        指定.ソースルート.display(),
        指定.出力ルート.display()
    );
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_asset_compiler", "--example", "compile_assets", "--"])
        .arg(指定.ソースルート)
        .arg(指定.出力ルート)
        .arg(指定.世界名);
    if let Some(個体数) = 指定.同居植生個体数 {
        コマンド.arg(個体数);
    }
    if let Some(名前) = 指定.テクスチャ格納方針の名前 {
        コマンド.args(super::texture_policy_name::選択肢の2語を組み立てる(名前));
    }
    起動結果を読む(コマンド.status())
}

fn 起動結果を読む(状態: std::io::Result<std::process::ExitStatus>) -> bool {
    match 状態 {
        Ok(終了状態) if 終了状態.success() => true,
        Ok(終了状態) => {
            eprintln!("[xtask] 実行時アセット生成が終了コード{終了状態}で失敗");
            false
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            false
        }
    }
}
