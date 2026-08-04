//! blitz_appが出す自動露出の1行を読み解く工程。受け取るのは標準出力、返すのはその行が持つ値の組である。
//!
//! 行が1本も無いことを失敗として扱うのは、報告が出なかった実行を「一致した」と読み替えないためである。

/// 自動露出の報告1行ぶんの値。判定はこの型を読むだけで済む。
pub(super) struct 自動露出の報告 {
    pub(super) ビン一致: bool,
    pub(super) 枠一致: bool,
    pub(super) 最初の食い違いビン: String,
    pub(super) ビンへ入った総数: String,
    pub(super) cpu添字の重み付き総和: String,
    pub(super) gpu添字の重み付き総和: String,
    pub(super) cpu目標補正段: String,
    pub(super) gpu目標補正段: String,
    pub(super) gpu補正段: String,
    pub(super) 初期化済み: bool,
    pub(super) 導出不能フレーム数: String,
    pub(super) 上端到達率: String,
    pub(super) 下端到達率: String,
    pub(super) 探り色のビン: String,
    pub(super) 探り色のcpu件数: String,
    pub(super) 探り色のgpu件数: String,
}

const 行の前置き: &str = "自動露出行 ";

pub(super) fn 読み解く(標準出力: &str, 条件名: &str) -> Result<自動露出の報告, String> {
    let 行 = 標準出力
        .lines()
        .find(|行| 行.trim_start().starts_with(行の前置き))
        .ok_or_else(|| format!("{条件名}の標準出力に自動露出行が1本も無い"))?;
    let 語一覧: Vec<&str> = 行.trim_start().trim_start_matches(行の前置き).split_whitespace().collect();
    Ok(自動露出の報告 {
        ビン一致: 真偽を読む(&語一覧, "ビン一致", 条件名)?,
        枠一致: 真偽を読む(&語一覧, "枠一致", 条件名)?,
        最初の食い違いビン: 値を読む(&語一覧, "最初の食い違いビン", 条件名)?,
        ビンへ入った総数: 値を読む(&語一覧, "ビンへ入った総数", 条件名)?,
        cpu添字の重み付き総和: 値を読む(&語一覧, "CPU添字の重み付き総和", 条件名)?,
        gpu添字の重み付き総和: 値を読む(&語一覧, "GPU添字の重み付き総和", 条件名)?,
        cpu目標補正段: 値を読む(&語一覧, "CPU目標補正段", 条件名)?,
        gpu目標補正段: 値を読む(&語一覧, "GPU目標補正段", 条件名)?,
        gpu補正段: 値を読む(&語一覧, "GPU補正段", 条件名)?,
        初期化済み: 真偽を読む(&語一覧, "初期化済み", 条件名)?,
        導出不能フレーム数: 値を読む(&語一覧, "導出不能フレーム数", 条件名)?,
        上端到達率: 値を読む(&語一覧, "上端到達率", 条件名)?,
        下端到達率: 値を読む(&語一覧, "下端到達率", 条件名)?,
        探り色のビン: 値を読む(&語一覧, "探り色のビン", 条件名)?,
        探り色のcpu件数: 値を読む(&語一覧, "探り色のCPU件数", 条件名)?,
        探り色のgpu件数: 値を読む(&語一覧, "探り色のGPU件数", 条件名)?,
    })
}

fn 値を読む(語一覧: &[&str], 鍵: &str, 条件名: &str) -> Result<String, String> {
    let 前置き = format!("{鍵}=");
    語一覧
        .iter()
        .find_map(|語| 語.strip_prefix(&前置き))
        .map(str::to_string)
        .ok_or_else(|| format!("{条件名}の自動露出行に「{鍵}」が無い"))
}

fn 真偽を読む(語一覧: &[&str], 鍵: &str, 条件名: &str) -> Result<bool, String> {
    let 値 = 値を読む(語一覧, 鍵, 条件名)?;
    match 値.as_str() {
        "真" => Ok(true),
        "偽" => Ok(false),
        _ => Err(format!("{条件名}の自動露出行の「{鍵}」が真でも偽でもない: {値}")),
    }
}
