//! `editor` コマンド: 編集サーバー(`editor_server`)とeditor_webの開発サーバーを併せて起動する。
//! 一方が終了したらもう一方を道連れに止める。Ctrl+Cでの正常終了までは面倒を見ない
//! （xtaskの責務は起動の一箇所化であり、シグナル配送の実装はしない）。

use std::{
    path::Path,
    process::{Child, Command},
    time::Duration,
};

pub fn 実行する() -> Result<(), String> {
    let リポジトリルート = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let editor_web = リポジトリルート.join("editor_web");

    let mut サーバー = 編集サーバーを起動する(&リポジトリルート)?;
    let mut web開発サーバー = editor_web開発サーバーを起動する試み(&editor_web);

    いずれかの終了を待つ(&mut サーバー, &mut web開発サーバー);
    後始末する(&mut サーバー, web開発サーバー);
    Ok(())
}

fn 編集サーバーを起動する(リポジトリルート: &Path) -> Result<Child, String> {
    Command::new("cargo")
        .args(["run", "-p", "editor_server"])
        .current_dir(リポジトリルート)
        .spawn()
        .map_err(|原因| format!("編集サーバーの起動に失敗した: {原因}"))
}

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
