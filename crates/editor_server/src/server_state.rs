//! axumの状態として配る、編集サーバーが読み書きするパスと保管庫の集まり。
//! パスの決定は起動時(`main.rs`のコンポジションルート)に閉じ、各ハンドラは
//! `サーバー状態`が持つ`ファイル保管庫`を通してだけファイルシステムへ触れる。
//! 実行時形式へ焼く局面は`bake_editor_world`が持つ(この型はその局面へ依存を貸すだけである)。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断2」

use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};

use axum::{http::StatusCode, response::Response};

use crate::building_grid_store::建物の格子の保存係;
use crate::failure_response::失敗応答を組み立てる;
use crate::project_info_contract::プロジェクト情報応答;
use crate::project_root::プロジェクトルート;
use crate::repository_root::リポジトリルート;
use crate::resource::建物外形カタログ;
use crate::storage::ファイル保管庫;

/// サーバー状態とは、リクエストハンドラへaxumが配る、編集サーバーの依存の集まりのことである。
#[derive(Clone)]
pub struct サーバー状態 {
    保管庫: ファイル保管庫,
    プロジェクト情報: プロジェクト情報応答,
    静的配信ディレクトリ: PathBuf,
    リポジトリルート: リポジトリルート,
    建物の格子の保存係: Arc<Mutex<建物の格子の保存係>>,
    ソースアセット書き出しの排他権: Arc<Mutex<()>>,
}

impl サーバー状態 {
    pub fn 生成する(
        リポジトリルート: &リポジトリルート,
        プロジェクトルート: &プロジェクトルート,
        建物の格子の保存係: 建物の格子の保存係,
    ) -> Self {
        Self {
            保管庫: ファイル保管庫::生成する(プロジェクトルート),
            プロジェクト情報: プロジェクト情報応答::生成する(
                プロジェクトルート.パス文字列(),
                プロジェクトルート.プロジェクト名(),
            ),
            静的配信ディレクトリ: リポジトリルート.静的配信ディレクトリ(),
            リポジトリルート: リポジトリルート.clone(),
            建物の格子の保存係: Arc::new(Mutex::new(建物の格子の保存係)),
            ソースアセット書き出しの排他権: Arc::new(Mutex::new(())),
        }
    }

    pub fn 保管庫(&self) -> &ファイル保管庫 {
        &self.保管庫
    }

    pub fn プロジェクト情報(&self) -> &プロジェクト情報応答 {
        &self.プロジェクト情報
    }

    pub fn 静的配信ディレクトリ(&self) -> &Path {
        &self.静的配信ディレクトリ
    }

    /// ソースアセットの書き出し先(`assets/<世界名>/`)を組み立てるための起点。
    /// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断5」
    pub fn リポジトリルート(&self) -> &リポジトリルート {
        &self.リポジトリルート
    }

    pub fn ソースアセット書き出しの排他権(&self) -> &Mutex<()> {
        &self.ソースアセット書き出しの排他権
    }

    /// 建物の格子の保存係を貸す。保存が台帳とカタログを同時に書き換えるため、読みも書きも1つの錠で守る。
    pub fn 建物の格子の保存係を借りる(&self) -> Result<MutexGuard<'_, 建物の格子の保存係>, Box<Response>> {
        self.建物の格子の保存係.lock().map_err(|誤り| {
            Box::new(失敗応答を組み立てる(
                StatusCode::INTERNAL_SERVER_ERROR,
                "内部エラー",
                誤り.to_string(),
            ))
        })
    }

    /// いま有効な建物外形カタログの写し。保存のたびに差し替わるため、呼び出しごとに錠を取って写しを作る。
    pub fn 建物外形カタログの写しを取る(&self) -> Result<建物外形カタログ, Box<Response>> {
        Ok(self.建物の格子の保存係を借りる()?.現在のカタログ().clone())
    }
}
