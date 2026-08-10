//! クラスタの割り当ての報告行の読み取り。受け取るのは標準出力、返すのは統計の値である。
//! 行の綴りは`crates/blitz_app/src/reports/cluster_assignment.rs`の出力と一致させている。
//!
//! 最後に現れた行を読むのは、先行フレームも書き出す実行では同じ行が2回出るためである。2回とも同じ値であることは
//! 読み戻しの決定性の判定が別に見ている。

const 語頭: &str = "クラスタ割り当て";

pub(super) struct 割り当ての統計 {
    pub(super) セル総数: u64,
    pub(super) 宣言光数: u64,
    pub(super) 光を持つセル数: u64,
    pub(super) 総件数: u64,
    pub(super) 最大件数: u64,
    pub(super) 平均件数: f64,
    pub(super) 絞りの比: f64,
    pub(super) 偽陽性件数: u64,
    pub(super) 偽陽性率: f64,
}

pub(super) fn 取り出す(標準出力: &str) -> Result<割り当ての統計, String> {
    let 行 = 標準出力
        .lines()
        .rfind(|行| 行.trim_start().starts_with(語頭))
        .ok_or_else(|| format!("出力に「{語頭}」で始まる行が無い"))?;
    Ok(割り当ての統計 {
        セル総数: 整数を読む(行, "セル総数")?,
        宣言光数: 整数を読む(行, "宣言光数")?,
        光を持つセル数: 整数を読む(行, "光を持つセル数")?,
        総件数: 整数を読む(行, "総件数")?,
        最大件数: 整数を読む(行, "最大件数")?,
        平均件数: 小数を読む(行, "平均件数")?,
        絞りの比: 小数を読む(行, "絞りの比")?,
        偽陽性件数: 整数を読む(行, "偽陽性件数")?,
        偽陽性率: 小数を読む(行, "偽陽性率")?,
    })
}

fn 値を切り出す<'a>(行: &'a str, 名前: &str) -> Result<&'a str, String> {
    行.split_whitespace()
        .find_map(|語| 語.strip_prefix(&format!("{名前}=")))
        .ok_or_else(|| format!("「{行}」に{名前}が無い"))
}

fn 整数を読む(行: &str, 名前: &str) -> Result<u64, String> {
    値を切り出す(行, 名前)?
        .parse()
        .map_err(|誤り| format!("「{行}」の{名前}を整数として読めない: {誤り}"))
}

fn 小数を読む(行: &str, 名前: &str) -> Result<f64, String> {
    値を切り出す(行, 名前)?
        .parse()
        .map_err(|誤り| format!("「{行}」の{名前}を小数として読めない: {誤り}"))
}
