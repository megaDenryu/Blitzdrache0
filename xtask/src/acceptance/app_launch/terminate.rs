//! 積み終えた起動を走らせて終わらせる局面。いつ呼ばれるかで積む局面と分かれる。積む口は起動のたびに何度でも
//! 呼ばれるのに対し、ここの2つはその起動でちょうど1回だけ呼ばれ、呼んだ時点でセッションが閉じる。
//!
//! どちらも最初に終了済みへ遷移させる。プロセスを起こせなかった実行もセッションとしては閉じており、
//! そこで`Drop`が閉じ忘れとして落とすと、本当の失敗の理由が破棄の失敗に隠れる。

use super::super::error::検収エラー;
use super::super::exit_report::終了時報告;
use super::アプリの起動;

impl アプリの起動 {
    /// 積み終えた指定で走らせ、標準出力も標準エラーもそのまま画面へ流したまま終わりを待つ。
    /// 人が画面を閉じるまで続く実行と、進み具合を見ながら通す長い実行がこれを通る。
    pub(in crate::acceptance) fn 画面へ流したまま走らせて終わりを待つ(mut self) -> Result<(), 検収エラー> {
        self.終了済みか = true;
        let 終了状態 = self.コマンド.status().map_err(|誤り| 検収エラー::アプリを起こせなかった {
            実行名: self.実行名.clone(),
            起こし方: self.起こし方.表示の綴り(),
            誤り,
        })?;
        if 終了状態.success() {
            return Ok(());
        }
        Err(検収エラー::アプリが失敗して終わった {
            実行名: self.実行名.clone(),
            終了状態: 終了状態.to_string(),
        })
    }

    /// 積み終えた指定で走らせ、終了時報告を得る。失敗した実行は報告を画面へ流してから落とす。
    pub(in crate::acceptance) fn 走らせて終了時報告を得る(mut self) -> Result<終了時報告, 検収エラー> {
        self.終了済みか = true;
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
            実行名: self.実行名.clone(),
            終了状態: 出力.status.to_string(),
        })
    }
}
