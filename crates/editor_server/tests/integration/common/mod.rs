//! 統合テスト共有ヘルパー: 実リポジトリを汚さないための一時プロジェクトの組み立て。
//! 各テストバイナリが個別に`mod common;`するため、そのバイナリが使わない項目は
//! 未使用警告になる。共有ヘルパーモジュールとして許容する。
#![allow(clippy::unwrap_used)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod building_grid_fixture;
mod building_outline_catalog_fixture;
mod chunk_height_cutout_fixture;
mod music_fixture;
mod scatter_export_fixture;
mod scatter_fixture;
mod source_asset_export_fixture;
mod world_layout;

use std::path::{Path, PathBuf};

use editor_server::{ファイル保管庫, プロジェクトルート, リポジトリルート};

pub use building_grid_fixture::{
    初期の格子のjson, 升目を空にした格子のjson, 外部アセットの置き場があるか, 格子の台帳を読む, 格子の建物を1件置いた構造, 試験の建物定義の識別子,
};
pub use building_outline_catalog_fixture::{カタログに無い識別子, 一間四方の家の識別子, 建物外形カタログを作る};
pub use chunk_height_cutout_fixture::{
    チャンク一辺頂点数, マザーの高さ, 大域を一意な値で用意する, 高さ一覧へ解く, 高さ格子を取得する
};
pub use music_fixture::{パターンの名乗り, 名乗り, 打点のない格子, 最初のパターン, 楽曲の例, 独自の進行};
pub use scatter_export_fixture::{
    散布を載せて書き出す, 書き出したチャンクソースを読む, 焼いたチャンクのバイト列を読む
};
pub use scatter_fixture::{
    個体の水平位置一覧, 建物が散布を避ける距離メートル, 建物までの距離, 散布の個体一覧を作る, 散布を載せたチャンク構造を保存する,
    検査のチャンク一辺メートル, 道路の中心線までの距離, 道路の散布除外バッファメートル,
};
pub use source_asset_export_fixture::{
    エディターの区画割り, フォックスのソースを配置する, マザーを一意な値で保存する, 地表層のタイルを配置する, 大域世界を保存する, 小さな区画割り,
    建物を据えられる区画割り, 零のマザーを保存する,
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
/// 使うため、静的配信ディレクトリ(editor_web/dist)は存在せず代替応答になるが、
/// API経路のテストには影響しない。
pub fn ルーターを作る(一時プロジェクト: &一時プロジェクト) -> editor_server::経路正規化アプリ {
    let カタログのファイル = blitz_asset_compiler::建物外形カタログのファイル::リポジトリルートから生成する(
        一時プロジェクト.リポジトリルート().パス(),
    );
    let 保存係 = match editor_server::建物の格子の保存係::起動時の材料から生成する(
        &一時プロジェクト.プロジェクトルート(),
        カタログのファイル,
        建物外形カタログを作る(),
    ) {
        Ok(保存係) => 保存係,
        Err(原因) => panic!("一時プロジェクトの建物の格子の台帳を読めない: {原因}"),
    };
    editor_server::ルーターを組み立てる(editor_server::サーバー状態::生成する(
        &一時プロジェクト.リポジトリルート(),
        &一時プロジェクト.プロジェクトルート(),
        保存係,
    ))
}

/// 一時プロジェクトを起点に`ファイル保管庫`を直接組み立てる(HTTPを経由しない保存往復テスト用)。
pub fn 保管庫を作る(識別子: &str) -> (一時プロジェクト, ファイル保管庫) {
    let 一時プロジェクト = 一時プロジェクト::生成する(識別子);
    let 保管庫 = ファイル保管庫::生成する(&一時プロジェクト.プロジェクトルート());
    (一時プロジェクト, 保管庫)
}
