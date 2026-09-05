//! 検証列の実行係。ログの出力係とログのパスを保持し、段を順に走らせて終了コードを返す。
//!
//! conformを関数の呼び出しでなく子プロセスとして走らせるのは、conformの出力もほかの段と同じ経路でログへ入れる
//! ためである。関数の呼び出しのままだと、その出力だけが端末へ直接出てログに残らない。
//!
//! 段が落ちたときの行と、全部通ったときの行の綴りは読む側との契約である。読む側はログをその行で探すため、
//! 綴りを変えると探せなくなる。参照: `.claude/agents/ビルド員.md`

use std::path::PathBuf;
use std::process::{Command, ExitCode};

use super::log_error::検証列の破れ;
use super::tee::端末とログの両方へ流す出力係;

/// cargoで走らせる段の一覧。実行時間の短い検査ほど前に置き、落ちる場合は早く落とす。fmtはコンパイルを伴わないためcheckより前に置く。
const 検証列の手順一覧: [(&str, &[&str]); 4] = [
    ("fmt", &["fmt", "--all", "--check"]),
    ("check", &["check", "--workspace"]),
    (
        "clippy",
        &[
            "clippy",
            "--all-targets",
            "--features",
            "editor_server/typescript",
            "--",
            "-D",
            "warnings",
        ],
    ),
    ("test", &["test", "--workspace", "--features", "editor_server/typescript"]),
];

pub struct 検証列の実行係 {
    出力係: 端末とログの両方へ流す出力係,
    ログのパス: PathBuf,
}

impl 検証列の実行係 {
    pub fn ログを開いて作る(ログのパス: PathBuf) -> Result<Self, 検証列の破れ> {
        let 出力係 = 端末とログの両方へ流す出力係::ログのファイルを開く(&ログのパス)?;
        Ok(Self { 出力係, ログのパス })
    }

    /// ログの場所を最初と最後に告げてから段を走らせる。端末の先頭と末尾のどちらを見てもログを開けるようにする。
    pub fn ログの場所を告げて全段を走らせる(&self) -> Result<ExitCode, 検証列の破れ> {
        self.ログの場所を告げる()?;
        let 終了コード = self.全段を走らせる()?;
        self.ログの場所を告げる()?;
        Ok(終了コード)
    }

    fn ログの場所を告げる(&self) -> Result<(), 検証列の破れ> {
        self.出力係.標準出力へ行を流す(&format!("[xtask] ログ: {}", self.ログのパス.display()))
    }

    fn 全段を走らせる(&self) -> Result<ExitCode, 検証列の破れ> {
        let 誰も読まない値 = 0;
        if !self.conformの段を走らせる()? {
            return Ok(ExitCode::FAILURE);
        }
        for (段の名前, cargo引数) in 検証列の手順一覧 {
            let mut 命令 = Command::new("cargo");
            命令.args(cargo引数);
            if !self.段を走らせて結果を告げる(段の名前, &format!("cargo {段の名前}"), &mut 命令)? {
                return Ok(ExitCode::FAILURE);
            }
        }
        self.出力係.標準出力へ行を流す("[xtask] 検証列すべて成功")?;
        Ok(ExitCode::SUCCESS)
    }

    fn conformの段を走らせる(&self) -> Result<bool, 検証列の破れ> {
        let 自分の実行ファイル = std::env::current_exe().map_err(検証列の破れ::自分の実行ファイルの場所を読めなかった)?;
        let mut 命令 = Command::new(自分の実行ファイル);
        命令.arg("conform");
        self.段を走らせて結果を告げる("conform", "conform", &mut 命令)
    }

    fn 段を走らせて結果を告げる(
        &self, 段の名前: &str, 実行の表示: &str, 命令: &mut Command
    ) -> Result<bool, 検証列の破れ> {
        self.出力係.標準出力へ行を流す(&format!("[xtask] {実行の表示} を実行"))?;
        if self.出力係.子プロセスの出力を流す(命令)? {
            return Ok(true);
        }
        self.出力係
            .標準エラーへ行を流す(&format!("[xtask] {段の名前} が失敗した。ここで中断する"))?;
        Ok(false)
    }
}
