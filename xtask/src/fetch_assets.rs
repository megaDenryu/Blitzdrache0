//! 標準サンプルglTFの取得(DamagedHelmet=M4、Fox=M8のアニメーション付きサンプル)。
//! xtaskはstdのみを保つため、HTTP取得はcurl.exe子プロセス(Windows 11同梱)に委ねる。
//!
//! 取得先の綴りを型の私有にして、取得済みかの判定と取得の実行だけを外へ出す。スモークが同じ綴りを
//! 持ち直していると、取得先を動かしたときに「取得はできるがステージが見つけられない」状態になる。
//!
//! 取得先はソースルートからの相対で持ち、置き場の組み立てはソースルートの型のメソッドが行う。
//! 取得先を裸の綴りで持って連結を書くと、取得先とアセットの読み手が別々の綴り方でルートを組むことになる。

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use blitz_asset_compiler::{ソースアセットの相対パス, ソースルート};

/// 本体リポジトリのソースルート。作業ディレクトリはリポジトリの根であり、取得先はここからの相対で決まる。
fn 本体のソースルート() -> ソースルート {
    ソースルート::生成する(PathBuf::from("assets"))
}

/// 取得する標準サンプル1件。どこから採ってどこへ置くかを持ち、その置き場所の綴りは外へ出さない。
pub struct 標準サンプルの取得対象 {
    取得先相対パス: ソースアセットの相対パス,
    取得元ディレクトリurl: &'static str,
}

pub const ヘルメットの取得対象: 標準サンプルの取得対象 = 標準サンプルの取得対象 {
    取得先相対パス: ソースアセットの相対パス::ヘルメット,
    取得元ディレクトリurl: "https://github.com/KhronosGroup/glTF-Sample-Assets/raw/main/Models/DamagedHelmet/glTF-Binary",
};

pub const フォックスの取得対象: 標準サンプルの取得対象 = 標準サンプルの取得対象 {
    取得先相対パス: ソースアセットの相対パス::フォックス,
    取得元ディレクトリurl: "https://github.com/KhronosGroup/glTF-Sample-Assets/raw/main/Models/Fox/glTF-Binary",
};

const 取得対象一覧: [&標準サンプルの取得対象; 2] = [&ヘルメットの取得対象, &フォックスの取得対象];

impl 標準サンプルの取得対象 {
    /// 既に取得してあるか。取得していない世界でステージを走らせないための判定である。
    pub fn 取得済みか(&self) -> bool {
        self.取得先の場所().is_file()
    }

    fn 取得先の場所(&self) -> PathBuf {
        本体のソースルート().宣言の相対パスが指すソース(self.取得先相対パス)
    }

    fn 取得する(&self) -> ExitCode {
        let 取得先パス = self.取得先の場所();
        if 取得先パス.exists() {
            println!("[xtask] 既に取得済み: {}", 取得先パス.display());
            return ExitCode::SUCCESS;
        }
        let 取得先ディレクトリ = 本体のソースルート().宣言の相対パスが指すソースを収める場所(self.取得先相対パス);
        if let Err(誤り) = std::fs::create_dir_all(&取得先ディレクトリ) {
            eprintln!("[xtask] 取得先ディレクトリの作成に失敗: {誤り}");
            return ExitCode::FAILURE;
        }
        let 取得元url = format!("{}/{}", self.取得元ディレクトリurl, self.取得先相対パス.末尾のファイル名().綴りを見せる());
        println!("[xtask] curl.exe -L -f -o {} {} を実行", 取得先パス.display(), 取得元url);
        let 起動結果 = Command::new("curl.exe")
            .args(["-L", "-f", "-o"])
            .arg(&取得先パス)
            .arg(&取得元url)
            .status();
        self.取得の結果を告げる(起動結果, &取得先パス)
    }

    fn 取得の結果を告げる(&self, 起動結果: std::io::Result<std::process::ExitStatus>, 取得先パス: &Path) -> ExitCode {
        match 起動結果 {
            Ok(終了状態) if 終了状態.success() => {
                println!("[xtask] 取得成功: {}", 取得先パス.display());
                ExitCode::SUCCESS
            }
            Ok(終了状態) => {
                eprintln!("[xtask] curl.exeが終了コード{終了状態}で失敗した");
                ExitCode::FAILURE
            }
            Err(起動誤り) => {
                eprintln!("[xtask] curl.exeの起動に失敗: {起動誤り}");
                ExitCode::FAILURE
            }
        }
    }
}

pub fn 実行する() -> ExitCode {
    for 対象 in 取得対象一覧 {
        if 対象.取得する() == ExitCode::FAILURE {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
