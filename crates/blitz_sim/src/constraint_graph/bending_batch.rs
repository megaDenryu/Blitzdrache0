//! 曲げ拘束のバッチ(判断4の密なバッチ)。参加する4点の添字と静的な引数(静止角・曲げのコンプライアンス)の一覧を持つ。
//! 生成が課す不変条件は、4点の添字が点の数の範囲内であること・4点が互いに別の点であること(同じ点があると三角形が潰れて法線が定まらず、
//! 反復の中で無言に無視され続ける)・本数がu32に収まること(GPUの添字が32ビットである)の3つである。
//! 彩色(色ごとの並べ替え)は`bending_coloring`が持ち、このバッチから作る。

use super::error::拘束グラフエラー;
use super::point_index::点添字;
use crate::xpbd::曲げ拘束の引数;

/// 曲げ拘束のバッチの1本。辺a・辺bが共有する辺、翼c・翼dがそれぞれの三角形の残りの点である(`xpbd::二面角の幾何`の並びと同じ)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 添字付き曲げ拘束 {
    pub 辺a: 点添字,
    pub 辺b: 点添字,
    pub 翼c: 点添字,
    pub 翼d: 点添字,
    pub 引数: 曲げ拘束の引数,
}

impl 添字付き曲げ拘束 {
    /// 4点のうち2つが同じ点である拘束は型付きエラーで拒む。
    pub fn 生成する(
        辺a: 点添字, 辺b: 点添字, 翼c: 点添字, 翼d: 点添字, 引数: 曲げ拘束の引数
    ) -> Result<Self, 拘束グラフエラー> {
        let 点一覧 = [辺a, 辺b, 翼c, 翼d];
        for (先, 点) in 点一覧.iter().enumerate() {
            if 点一覧[先 + 1..].contains(点) {
                return Err(拘束グラフエラー::曲げ拘束が同じ点を持つ { 点: *点 });
            }
        }
        Ok(Self {
            辺a, 辺b, 翼c, 翼d, 引数
        })
    }

    /// 参加する4点を辺a・辺b・翼c・翼dの順で返す。
    pub fn 点一覧(&self) -> [点添字; 4] {
        [self.辺a, self.辺b, self.翼c, self.翼d]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct 曲げ拘束のバッチ {
    点の数: usize,
    拘束一覧: Vec<添字付き曲げ拘束>,
}

impl 曲げ拘束のバッチ {
    /// 範囲外の点添字とu32に収まらない本数を型付きエラーで拒む。0本のバッチも許す(一辺2の格子には内側の辺が斜め1本しか無い)。
    pub fn 生成する(点の数: usize, 拘束一覧: Vec<添字付き曲げ拘束>) -> Result<Self, 拘束グラフエラー> {
        if u32::try_from(拘束一覧.len()).is_err() {
            return Err(拘束グラフエラー::曲げ拘束の数が過大 {
                拘束の数: 拘束一覧.len()
            });
        }
        for 拘束 in &拘束一覧 {
            for 添字 in 拘束.点一覧() {
                if 添字.配列添字() >= 点の数 {
                    return Err(拘束グラフエラー::曲げ拘束の点添字が範囲外 { 添字, 点の数 });
                }
            }
        }
        Ok(Self { 点の数, 拘束一覧 })
    }

    pub fn 拘束一覧(&self) -> &[添字付き曲げ拘束] {
        &self.拘束一覧
    }

    pub fn 拘束の数(&self) -> usize {
        self.拘束一覧.len()
    }

    // 各拘束へ色(0始まり)を貪欲に割り当てた並び。点ごとに使用済みの色を持ち、4点の和集合に無い最小の色を選ぶ。決定的である。
    pub(super) fn 拘束へ貪欲に色を割り当てる(&self) -> Vec<usize> {
        let mut 点が使った色: Vec<Vec<usize>> = vec![Vec::new(); self.点の数];
        let mut 色一覧 = Vec::with_capacity(self.拘束一覧.len());
        for 拘束 in &self.拘束一覧 {
            let mut 色 = 0;
            while 拘束.点一覧().iter().any(|点| 点が使った色[点.配列添字()].contains(&色)) {
                色 += 1;
            }
            for 点 in 拘束.点一覧() {
                点が使った色[点.配列添字()].push(色);
            }
            色一覧.push(色);
        }
        色一覧
    }
}
