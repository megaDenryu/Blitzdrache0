//! 統合テスト共有ヘルパー: 実リポジトリを汚さないための一時プロジェクトの組み立て。
//! 各テストバイナリが個別に`mod common;`するため、そのバイナリが使わない項目は
//! 未使用警告になる。共有ヘルパーモジュールとして許容する。
#![allow(clippy::unwrap_used)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod source_asset_export_fixture;
mod world_layout;

use std::path::{Path, PathBuf};

use editor_server::{ファイル保管庫, プロジェクトルート, リポジトリルート};

pub use source_asset_export_fixture::{
    エディターの区画割り, マザーを一意な値で保存する, 大域世界を保存する, 小さな区画割り, 零のマザーを保存する
};
pub use world_layout::{区画割りJson, 区画割りを保存する};

/// 一時プロジェクトとは、実リポジトリ(`editor_data/`)を汚さずに
/// サーバー状態を組み立てるための使い捨てディレクトリのことである。ドロップ時に自身を削除する。
pub struct 一時プロジェクト {
    ルート: PathBuf,
}

impl 一時プロジェクト {
    pub fn 生成する(識別子: &str) -> Self {
        let ルート = std::env::temp_dir().join(format!("editor_server_test_{識別子}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&ルート);
        std::fs::create_dir_all(ルート.join("editor_data")).unwrap();
        Self { ルート }
    }

    pub fn ルート(&self) -> &Path {
        &self.ルート
    }

    pub fn プロジェクトルート(&self) -> プロジェクトルート {
        プロジェクトルート::生成する(self.ルート.clone())
    }

    pub fn リポジトリルート(&self) -> リポジトリルート {
        リポジトリルート::生成する(self.ルート.clone())
    }
}

impl Drop for 一時プロジェクト {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.ルート);
    }
}

/// 一時プロジェクトを起点にルーターを組み立てる。リポジトリルートも同じ一時ディレクトリを
/// 使うため、静的配信ディレクトリ(editor_web/dist)は存在せずフォールバック応答になるが、
/// API経路のテストには影響しない。
pub fn ルーターを作る(一時プロジェクト: &一時プロジェクト) -> editor_server::経路正規化アプリ {
    editor_server::ルーターを組み立てる(editor_server::サーバー状態::生成する(
        &一時プロジェクト.リポジトリルート(),
        &一時プロジェクト.プロジェクトルート(),
    ))
}

/// 一時プロジェクトを起点に`ファイル保管庫`を直接組み立てる(HTTPを経由しない保存往復テスト用)。
pub fn 保管庫を作る(識別子: &str) -> (一時プロジェクト, ファイル保管庫) {
    let 一時プロジェクト = 一時プロジェクト::生成する(識別子);
    let 保管庫 = ファイル保管庫::生成する(&一時プロジェクト.プロジェクトルート());
    (一時プロジェクト, 保管庫)
}
