//! アプリの起動: 1回ぶんのプロセス起動を、引数を積んでから走らせるまで持つセッション型。
//!
//! 起こす手順をコールバックで挟まずこの型で表すのは、積む・走らせる・終了状態を判定するの3手が
//! その場で1回ずつ実行される括りであるためである。積み終える前に走らせる形は、この型を通す限り書けない。
//!
//! 引数を積む口を用途ごとに分けるのは、綴り(`--asset-root`等)をこの型の中だけに置くためである。
//! 汎用の「引数を足す」だけを出すと、同じ綴りが呼び出し側の数だけ書かれる。

use std::process::Command;

use super::app_executable::アプリの起こし方;
use super::dump_format::書き出しの形式;
use super::error::検収エラー;
use super::exit_report::終了時報告;
use super::launch_specification::アプリの起動指定;
use super::readback_dump::読み戻しの書き出し先;
use super::run_name::検収の実行名;
use super::runtime_asset_root::実行時アセットルート;

#[must_use]
pub(super) struct アプリの起動 {
    起こし方: アプリの起こし方,
    実行名: 検収の実行名,
    コマンド: Command,
}

impl アプリの起動 {
    pub(super) fn 始める(起こし方: アプリの起こし方, 実行名: 検収の実行名) -> Self {
        let コマンド = 起こし方.コマンドを作る();
        Self {
            起こし方, 実行名, コマンド
        }
    }

    pub(super) fn 起動指定を積む(&mut self, 指定: &アプリの起動指定) {
        指定.コマンドへ並べる(&mut self.コマンド);
    }

    pub(super) fn 読む世界の置き場を積む(&mut self, アセットルート: &実行時アセットルート) {
        self.コマンド.arg("--asset-root").arg(アセットルート.起動引数として渡す綴り());
    }

    /// 最終フレームを指定の形式で書き出させる。
    pub(super) fn 書き出し先を積む(&mut self, 書き出し先: &読み戻しの書き出し先, 形式: 書き出しの形式) {
        self.コマンド.arg(形式.選択肢の綴り()).arg(書き出し先.起動引数として渡す綴り());
    }

    /// 世界を読まない報告の求めを積む。シーンも枚数も持たない起動がこの口を通る。
    pub(super) fn 選択肢を積む(&mut self, 綴り一覧: &[&str]) {
        self.コマンド.args(綴り一覧);
    }

    /// 積み終えた指定で走らせ、標準出力も標準エラーもそのまま画面へ流したまま終わりを待つ。
    /// 人が画面を閉じるまで続く実行がこれを通る。報告を捕まえないのは、遊んでいる最中の出力を溜めずに見せるためである。
    pub(super) fn 画面へ流したまま走らせて終わりを待つ(mut self) -> Result<(), 検収エラー> {
        let 終了状態 = self.コマンド.status().map_err(|誤り| 検収エラー::アプリを起こせなかった {
            実行名: self.実行名.clone(),
            起こし方: self.起こし方.表示の綴り(),
            誤り,
        })?;
        if 終了状態.success() {
            return Ok(());
        }
        Err(検収エラー::アプリが失敗して終わった {
            実行名: self.実行名,
            終了状態: 終了状態.to_string(),
        })
    }

    /// 積み終えた指定で走らせ、終了時報告を得る。失敗した実行は報告を画面へ流してから落とす。
    pub(super) fn 走らせて終了時報告を得る(mut self) -> Result<終了時報告, 検収エラー> {
        let 出力 = self.コマンド.output().map_err(|誤り| 検収エラー::アプリを起こせなかった {
            実行名: self.実行名.clone(),
            起こし方: self.起こし方.表示の綴り(),
            誤り,
        })?;
        let 報告 = 終了時報告::取り込む(
            &self.実行名,
            String::from_utf8_lossy(&出力.stdout).into_owned(),
            String::from_utf8_lossy(&出力.stderr).into_owned(),
        );
        if 出力.status.success() {
            return Ok(報告);
        }
        報告.画面へ流す();
        報告.標準エラーを画面へ流す();
        Err(検収エラー::アプリが失敗して終わった {
            実行名: self.実行名,
            終了状態: 出力.status.to_string(),
        })
    }
}
