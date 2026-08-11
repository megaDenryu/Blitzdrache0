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

/// 標準出力を取り込んで返す形で焼く。増分が効いたかを外から見る材料が、コンパイラの報告の行だけであるためである。
/// 取り込んだ内容はそのまま表示するため、画面に出るものは流したときと変わらない。
pub(super) fn アセットコンパイラを起動して標準出力を取り込む(
    指定: &実行時形式を焼く指定
) -> Result<String, String> {
    これから焼く対象を告げる(指定);
    let 出力 = 起動するコマンドを組み立てる(指定)
        .output()
        .map_err(|誤り| format!("cargoの起動に失敗: {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    print!("{標準出力}");
    eprint!("{}", String::from_utf8_lossy(&出力.stderr));
    if 出力.status.success() {
        Ok(標準出力)
    } else {
        Err(format!("実行時アセット生成が終了コード{}で失敗", 出力.status))
    }
}

pub(super) fn アセットコンパイラを起動して実行時形式を焼く(指定: &実行時形式を焼く指定) -> bool {
    これから焼く対象を告げる(指定);
    コンパイラの起動結果を成否へ読む(起動するコマンドを組み立てる(指定).status())
}

fn これから焼く対象を告げる(指定: &実行時形式を焼く指定) {
    println!(
        "[xtask] 実行時アセット生成({}{}{}): {} -> {}",
        指定.世界名,
        指定.同居植生個体数.map_or_else(String::new, |個体数| format!(", 同居植生{個体数}体")),
        指定.テクスチャ格納方針の名前.map_or_else(String::new, |名前| format!(", 格納方針{名前}")),
        指定.ソースルート.display(),
        指定.出力ルート.display()
    );
}

fn 起動するコマンドを組み立てる(指定: &実行時形式を焼く指定) -> Command {
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
        コマンド.args(super::texture_policy_name::方針の選択肢の2語を組み立てる(名前));
    }
    コマンド
}

fn コンパイラの起動結果を成否へ読む(状態: std::io::Result<std::process::ExitStatus>) -> bool {
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
