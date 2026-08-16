//! カタログの組み上げの失敗。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。
//!
//! 対の相手の不足を1件ずつでなく一覧で持つのは、この誤りの読み手が部品を作るBlender側の作者であり、
//! 1件直して再実行するたびに次の1件が出る形だと、直しの往復がカタログの件数だけ要るためである。

use std::fmt;

use thiserror::Error;

use crate::joint::{接合点名, 接合種別};

use super::part_id::部品ID;

/// 対の相手がカタログに無い接合点1件。どの部品のどの接合点が、どの種別の相手を探して見つからなかったかを持つ。
#[derive(Debug, Clone, PartialEq)]
pub struct 対の不足 {
    pub 部品の識別子: 部品ID,
    pub 接合点の名前: 接合点名,
    pub 種別: 接合種別,
}

impl fmt::Display for 対の不足 {
    fn fmt(&self, 出力: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            出力,
            "{}の接合点「{}」が{}を宣言しているが、対になる{}がカタログのどの部品にも無い",
            self.部品の識別子,
            self.接合点の名前,
            self.種別,
            self.種別.対になる種別()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum カタログエラー {
    #[error("部品IDが空である")]
    部品IDが空,

    #[error("部品IDが重なっている: {部品の識別子}")]
    部品IDが重複している { 部品の識別子: 部品ID },

    #[error("対の相手がカタログに無い接合点が{}件ある\n{}", .不足一覧.len(), 不足一覧を綴る(.不足一覧))]
    対の相手がカタログに無い { 不足一覧: Vec<対の不足> },

    #[error("境界箱の最小が最大を超えている(軸{軸})")]
    境界箱の最小が最大を超える { 軸: usize },
}

fn 不足一覧を綴る(不足一覧: &[対の不足]) -> String {
    不足一覧.iter().map(|不足| format!("  {不足}")).collect::<Vec<String>>().join("\n")
}
