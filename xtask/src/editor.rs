//! `editor` コマンド: 編集サーバー(`editor_server`)とeditor_webの開発サーバーを併せて起動する。
//! 一方が終了したらもう一方を道連れに止め、Ctrl+Cで割り込まれた場合も同じ後始末を通す。
//! 止め方は子孫まで含めた木ごとの終了であり、その理由と手段は`process_id.rs`が持つ。
//! `--project <ルート>`を受け取った場合はそのまま`editor_server`へ引き渡す
//! (参照: `crates/editor_server/src/project_root.rs`)。検証用の使い捨てルートを渡すことで、
//! E2Eや検証が本物の`editor_data`を汚さずに済む。

use std::{path::Path, process::Command, sync::Arc, time::Duration};

use self::{
    dev_server_port::開発サーバーの待ち受け口, process_tree::子プロセスの木, project_root::プロジェクトルート, shutdown_registry::停止台帳,
    web_root::エディター画面の置き場,
};

pub(crate) mod building_outline_catalog;
mod dev_server_port;
mod interrupt;
mod process_id;
mod process_tree;
pub(crate) mod project_root;
mod shutdown_registry;
mod web_root;

pub fn エディターサーバーを起動する(追加引数: &[String]) -> Result<(), String> {
    let リポジトリルート = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let 画面の置き場 = エディター画面の置き場::リポジトリルートから求める(&リポジトリルート);
    let プロジェクトルート = プロジェクトルート::引数から解く(追加引数, &リポジトリルート);

    let カタログ = building_outline_catalog::既定のファイルへ書き出す(&リポジトリルート, &プロジェクトルート)?;
    println!("建物外形カタログ: {}", カタログ.display());
    if let Some(待ち受け口) = 開発サーバーの待ち受け口::vite設定から読み取る(&画面の置き場) {
        待ち受け口.塞がっているなら残りの掃除を案内する();
    }

    let 台帳 = Arc::new(停止台帳::空で作る());
    interrupt::割り込みを捕らえて台帳の木を終わらせるようにする(Arc::clone(&台帳));

    let mut サーバー = 編集サーバーを起動する(&リポジトリルート, 追加引数)?;
    台帳.木を登録する(サーバー.番号());
    let mut web開発サーバー = 画面の置き場.開発サーバーを起動する試み();
    if let Some(木) = web開発サーバー.as_ref() {
        台帳.木を登録する(木.番号());
    }

    いずれかの終了を待つ(&mut サーバー, &mut web開発サーバー);
    後始末する(&mut サーバー, web開発サーバー);
    Ok(())
}

fn 編集サーバーを起動する(リポジトリルート: &Path, 追加引数: &[String]) -> Result<子プロセスの木, String> {
    Command::new("cargo")
        .args(["run", "-p", "editor_server"])
        .args(editor_serverへ渡す引数を組み立てる(追加引数))
        .current_dir(リポジトリルート)
        .spawn()
        .map(子プロセスの木::起動済みの子から作る)
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
mod tests;

fn いずれかの終了を待つ(サーバー: &mut 子プロセスの木, web開発サーバー: &mut Option<子プロセスの木>) {
    loop {
        if let Some(状態) = サーバー.終わっていれば終了状態を返す() {
            println!("編集サーバーが終了した（状態: {状態}）");
            return;
        }
        if let Some(木) = web開発サーバー.as_mut()
            && let Some(状態) = 木.終わっていれば終了状態を返す()
        {
            println!("editor_webの開発サーバーが終了した（状態: {状態}）");
            return;
        }
        std::thread::sleep(Duration::from_millis(300));
    }
}

fn 後始末する(サーバー: &mut 子プロセスの木, web開発サーバー: Option<子プロセスの木>) {
    サーバー.木ごと終わらせて見送る();
    if let Some(mut 木) = web開発サーバー {
        木.木ごと終わらせて見送る();
    }
}
