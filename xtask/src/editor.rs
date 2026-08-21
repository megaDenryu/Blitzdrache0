//! `editor` コマンド: 編集サーバー(`editor_server`)とeditor_webの開発サーバーを併せて起動する。
//! 一方が終了したらもう一方を道連れに止める。Ctrl+Cでの正常終了までは面倒を見ない
//! （xtaskの責務は起動の一箇所化であり、シグナル配送の実装はしない）。
//! `--project <ルート>`を受け取った場合はそのまま`editor_server`へ引き渡す
//! (参照: `crates/editor_server/src/project_root.rs`)。検証用の使い捨てルートを渡すことで、
//! E2Eや検証が本物の`editor_data`を汚さずに済む。

use std::{
    path::Path,
    process::{Child, Command},
    time::Duration,
};

pub fn 実行する(追加引数: &[String]) -> Result<(), String> {
    let リポジトリルート = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let editor_web = リポジトリルート.join("editor_web");

    let カタログ = super::building_outline_catalog::既定のファイルへ書き出す(&リポジトリルート)?;
    println!("建物外形カタログ: {}", カタログ.display());

    let mut サーバー = 編集サーバーを起動する(&リポジトリルート, 追加引数)?;
    let mut web開発サーバー = editor_web開発サーバーを起動する試み(&editor_web);

    いずれかの終了を待つ(&mut サーバー, &mut web開発サーバー);
    後始末する(&mut サーバー, web開発サーバー);
    Ok(())
}

fn 編集サーバーを起動する(リポジトリルート: &Path, 追加引数: &[String]) -> Result<Child, String> {
    Command::new("cargo")
        .args(["run", "-p", "editor_server"])
        .args(editor_serverへ渡す引数を組み立てる(追加引数))
        .current_dir(リポジトリルート)
        .spawn()
        .map_err(|原因| format!("編集サーバーの起動に失敗した: {原因}"))
}

/// `cargo run`自身の引数と`editor_server`本体への引数を`--`で区切る。追加引数が無ければ
/// `--`ごと付けない(空の`--`が付いても実害は無いが、無駄な引数を渡さないほうが素直である)。
fn editor_serverへ渡す引数を組み立てる(追加引数: &[String]) -> Vec<String> {
    if 追加引数.is_empty() {
        return Vec::new();
    }
    let mut 結果 = vec!["--".to_string()];
    結果.extend(追加引数.iter().cloned());
    結果
}

#[cfg(test)]
#[path = "editor/tests.rs"]
mod tests;

fn editor_web開発サーバーを起動する試み(editor_web: &Path) -> Option<Child> {
    if !editor_web.join("node_modules").is_dir() {
        println!("editor_web/node_modules が無い。次を実行してから改めて `cargo xtask editor` を起動する: (cd editor_web && npm install)");
        println!("編集サーバーだけを起動して続ける。");
        return None;
    }
    let npmプログラム名 = if cfg!(windows) { "npm.cmd" } else { "npm" };
    match Command::new(npmプログラム名).args(["run", "dev"]).current_dir(editor_web).spawn() {
        Ok(子) => Some(子),
        Err(原因) => {
            println!("editor_webの開発サーバーを起動できない: {原因}");
            println!("Node.jsとnpmを導入したうえで、editor_webで `npm run dev` を手で実行する。");
            None
        }
    }
}

fn いずれかの終了を待つ(サーバー: &mut Child, web開発サーバー: &mut Option<Child>) {
    loop {
        if let Ok(Some(状態)) = サーバー.try_wait() {
            println!("編集サーバーが終了した（状態: {状態}）");
            return;
        }
        if let Some(子) = web開発サーバー.as_mut()
            && let Ok(Some(状態)) = 子.try_wait()
        {
            println!("editor_webの開発サーバーが終了した（状態: {状態}）");
            return;
        }
        std::thread::sleep(Duration::from_millis(300));
    }
}

fn 後始末する(サーバー: &mut Child, web開発サーバー: Option<Child>) {
    let _ = サーバー.kill();
    let _ = サーバー.wait();
    if let Some(mut 子) = web開発サーバー {
        let _ = 子.kill();
        let _ = 子.wait();
    }
}
