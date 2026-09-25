//! 規約適合検査の報告。違反の一覧と警告の一覧を持ち、表示と終了コードを決める。
//! 終了コードを決めるのは違反の一覧だけであり、警告は表示するが終了コードを変えない。

use std::process::ExitCode;

use super::violation::違反;
use super::warning::警告;

pub struct 検査の報告 {
    違反一覧: Vec<違反>,
    警告一覧: Vec<警告>,
}

impl 検査の報告 {
    pub const fn 生成する(違反一覧: Vec<違反>, 警告一覧: Vec<警告>) -> Self {
        Self { 違反一覧, 警告一覧 }
    }

    /// 2つの報告の違反と警告をそれぞれ繋いだ報告。
    pub fn 合わせる(mut self, 他: Self) -> Self {
        self.違反一覧.extend(他.違反一覧);
        self.警告一覧.extend(他.警告一覧);
        self
    }

    /// 警告を見出しの後ろに並べてから、違反の有無で成功か失敗かを表示し、その終了コードを返す。最後の行が成否の行になるように警告を先に出す。
    pub fn 表示して終了コードを返す(self) -> ExitCode {
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
