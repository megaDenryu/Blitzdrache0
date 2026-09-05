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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod closing_tests;

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
    ///
    /// ログを開いた後の破れをここで終了コードへ畳んで返すのは、呼び出し側が端末へ直に書く経路を残すと、
    /// その本文がログへ入らず端末の最後の行もログのパスでなくなるためである。参照: PR #83のレビュー
    pub fn ログの場所を告げて全段を走らせる(&self) -> ExitCode {
        let 結果 = self.ログの場所を告げてから全段を走らせる();
        self.結末を締めて終了コードへ写す(結果)
    }

    fn ログの場所を告げてから全段を走らせる(&self) -> Result<ExitCode, 検証列の破れ> {
        self.ログの場所を告げる()?;
        self.全段を走らせる()
    }

    /// 成功・段の失敗・ログを開いた後の破れのどれでも、最後にログのパスを告げてから終了コードへ写す。
    /// 締めの書き込み自体が失敗しても元の結果を握り潰さないため、ここでは早期に返さない。
    ///
    /// 端末への複製が破れたまま終わったなら、段が全部通っていても失敗にする。読む側が端末で見た流れは
    /// 途中で切れており、それを成功として渡すと欠けた流れを全部と誤読させるためである。
    fn 結末を締めて終了コードへ写す(&self, 結果: Result<ExitCode, 検証列の破れ>) -> ExitCode {
        if let Err(破れ) = &結果 {
            let _ = self.出力係.標準エラーへ行を流す(&format!("[xtask] 検証列を走らせられなかった: {破れ}"));
        }
        let 端末の破れ = self.出力係.端末への複製の破れ();
        if let Some(説明) = &端末の破れ {
            let _ = self
                .出力係
                .標準エラーへ行を流す(&format!("[xtask] 端末への複製が破れたまま終わった: {説明}"));
        }
        let _ = self.ログの場所を告げる();
        if 端末の破れ.is_some() {
            return ExitCode::FAILURE;
        }
        結果.unwrap_or(ExitCode::FAILURE)
    }

    fn ログの場所を告げる(&self) -> Result<(), 検証列の破れ> {
        self.出力係.標準出力へ行を流す(&format!("[xtask] ログ: {}", self.ログのパス.display()))
    }

    fn 全段を走らせる(&self) -> Result<ExitCode, 検証列の破れ> {
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
