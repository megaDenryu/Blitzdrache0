//! 遠景を沈める量の分布と、詳細面との最小隙間の報告。

use blitz_math::メートル;

use super::deviation::セルごとの最大正偏差;
use super::error::遠景コンパイルエラー;
use super::gap::隙間の集計;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 遠景の沈み統計 {
    pub セル数: usize,
    pub 最大偏差: メートル,
    pub 中央値: メートル,
    pub p95: メートル,
    pub p99: メートル,
    pub 最小隙間: メートル,
    pub 正でない隙間数: usize,
    pub 安全幅: メートル,
}

impl 遠景の沈み統計 {
    pub(super) fn 求める(
        偏差: &セルごとの最大正偏差, 隙間: &隙間の集計, 安全幅: メートル
    ) -> Result<Self, 遠景コンパイルエラー> {
        let 並び = 偏差.小さい順に並べた写し();
        let 最大偏差 = match 並び.last() {
            Some(値) => *値,
            None => メートル::生成する(0.0),
        };
        隙間.正でない隙間があれば失敗にする()?;
        Ok(Self {
            セル数: 並び.len(),
            最大偏差,
            中央値: 百分位(&並び, 50),
            p95: 百分位(&並び, 95),
            p99: 百分位(&並び, 99),
            最小隙間: 隙間.最小隙間(),
            正でない隙間数: 隙間.正でない隙間数(),
            安全幅,
        })
    }

    /// 検収が読む報告の1行。
    ///
    /// 注意: 文字列は数値の単位型を持てないため、統計の値をここで生の実数へ戻す。
    /// 綴りと桁数は検収との契約であり、勝手に変えると読み取りが黙って外れる。
    /// 参照: `xtask/src/game_fox_tour/map_generation_check/tally_line.rs`
    pub fn 報告の行を作る(self) -> String {
        format!(
            "遠景沈み 安全幅={:.3}m セル数={} 最大偏差={:.3}m 中央値={:.3}m p95={:.3}m p99={:.3}m 最小隙間={:.6}m 非正隙間={}",
            self.安全幅.値(),
            self.セル数,
            self.最大偏差.値(),
            self.中央値.値(),
            self.p95.値(),
            self.p99.値(),
            self.最小隙間.値(),
            self.正でない隙間数
        )
    }
}

fn 百分位(並び: &[メートル], 百分率: usize) -> メートル {
    if 並び.is_empty() {
        return メートル::生成する(0.0);
    }
    let 添字 = (並び.len() - 1) * 百分率 / 100;
    match 並び.get(添字) {
        Some(値) => *値,
        None => メートル::生成する(0.0),
    }
}
