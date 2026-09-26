//! 規約適合検査の報告。違反の一覧と警告の一覧を持ち、表示と終了コードを決める。
//! 終了コードを決めるのは違反の一覧だけであり、警告は表示するが終了コードを変えない。
//! 検査がGraphiteの生成物と認めて対象から外したファイルも、外した検査ごとに件数と名前を表示する。終了コードは変えない。

use std::process::ExitCode;

use super::graphiteのコード::対象外にした生成物;
use super::violation::違反;
use super::warning::警告;

pub struct 検査の報告 {
    違反一覧: Vec<違反>,
    警告一覧: Vec<警告>,
    対象外にした生成物一覧: Vec<対象外にした生成物>,
}

impl 検査の報告 {
    pub const fn 生成する(違反一覧: Vec<違反>, 警告一覧: Vec<警告>) -> Self {
        Self {
            違反一覧,
            警告一覧,
            対象外にした生成物一覧: Vec::new(),
        }
    }

    /// 検査が対象から外した生成物を足した報告。
    pub fn 対象外にした生成物を足す(mut self, 外した生成物: 対象外にした生成物) -> Self {
        self.対象外にした生成物一覧.push(外した生成物);
        self
    }

    /// 警告の一覧。試験が、検査の工程が報告へ載せた警告を読むための口である。
    #[cfg(test)]
    pub fn 警告一覧(&self) -> &[警告] {
        &self.警告一覧
    }

    /// 対象から外した生成物の一覧。試験が、検査の工程が報告へ載せた外した生成物を読むための口である。
    #[cfg(test)]
    pub fn 対象外にした生成物一覧(&self) -> &[対象外にした生成物] {
        &self.対象外にした生成物一覧
    }

    /// 2つの報告の違反と警告をそれぞれ繋いだ報告。
    pub fn 合わせる(mut self, 他: Self) -> Self {
        self.違反一覧.extend(他.違反一覧);
        self.警告一覧.extend(他.警告一覧);
        self.対象外にした生成物一覧.extend(他.対象外にした生成物一覧);
        self
    }

    /// 対象から外した生成物と警告を先に並べてから、違反の有無で成功か失敗かを表示し、その終了コードを返す。最後の行が成否の行になるように成否を最後に出す。
    pub fn 表示して終了コードを返す(self) -> ExitCode {
        for 外した生成物 in &self.対象外にした生成物一覧 {
            println!("{外した生成物}");
        }
        if !self.警告一覧.is_empty() {
            println!("[xtask] conform警告: 警告{}件(警告は終了コードを変えない)", self.警告一覧.len());
            for 警告 in &self.警告一覧 {
                println!("警告: {警告}");
            }
        }
        if self.違反一覧.is_empty() {
            println!("[xtask] conform成功: 違反0件");
            return ExitCode::SUCCESS;
        }
        println!("[xtask] conform失敗: 違反{}件", self.違反一覧.len());
        for 違反 in &self.違反一覧 {
            println!("{違反}");
        }
        ExitCode::FAILURE
    }
}
