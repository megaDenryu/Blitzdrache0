//! ブラウザ側のエディター画面(`editor_web`)が住む置き場。担当するのは、この置き場に何が在るか
//! (viteの設定・依存の導入先)と、ここで開発サーバーをどう起動するかである。
//! 置き場の中の綴りを型の中へ閉じ、呼び出し側へ散らさない。

use std::path::{Path, PathBuf};
use std::process::Command;

use super::process_tree::子プロセスの木;

pub(crate) struct エディター画面の置き場 {
    パス: PathBuf,
}

impl エディター画面の置き場 {
    pub(crate) fn リポジトリルートから求める(リポジトリルート: &Path) -> Self {
        Self {
            パス: リポジトリルート.join("editor_web"),
        }
    }

    /// viteの設定ファイル。開発サーバーの待ち受け口の番号の正本がここに在る。
    pub(crate) fn vite設定のパス(&self) -> PathBuf {
        self.パス.join("vite.config.ts")
    }

    /// 依存の導入が済んでいなければ開発サーバーは起動できないため、起動を試みる前に見る。
    fn 依存の導入が済んでいるか(&self) -> bool {
        self.パス.join("node_modules").is_dir()
    }

    pub(crate) fn 開発サーバーを起動する試み(&self) -> Option<子プロセスの木> {
        if !self.依存の導入が済んでいるか() {
            println!("editor_web/node_modules が無い。次を実行してから改めて `cargo xtask editor` を起動する: (cd editor_web && npm install)");
            println!("編集サーバーだけを起動して続ける。");
            return None;
        }
        let npmプログラム名 = if cfg!(windows) { "npm.cmd" } else { "npm" };
        match Command::new(npmプログラム名).args(["run", "dev"]).current_dir(&self.パス).spawn() {
            Ok(子) => Some(子プロセスの木::起動済みの子から作る(子)),
            Err(原因) => {
                println!("editor_webの開発サーバーを起動できない: {原因}");
                println!("Node.jsとnpmを導入したうえで、editor_webで `npm run dev` を手で実行する。");
                None
            }
        }
    }
}
