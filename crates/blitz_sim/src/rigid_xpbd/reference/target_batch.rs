//! 剛体と世界の点の密なバッチ(判断10): 剛体の目標拘束の静的な引数(剛体の識別子・局所点・コンプライアンス)と毎刻みの入力(目標位置)と一細分内の乗数を同じ添字で持つ。

use blitz_math::{ワールド, 位置};

use super::super::body_target_constraint::{剛体の目標拘束の一刻みの係数, 剛体の目標拘束の引数};
use super::super::body_target_result::剛体の目標拘束の一回の射影の結果;
use super::super::error::剛体の参照計算エラー;
use super::substep_body::細分の中の剛体;
use crate::rigid_body::剛体の識別子;
use crate::xpbd::{ラグランジュ乗数, 刻み幅};

/// バッチの1本。目標位置は世界固定点として持つ(動く目標は毎刻みの入力の確定で差し替える)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 添字付き剛体の目標拘束 {
    pub 剛体: 剛体の識別子,
    pub 引数: 剛体の目標拘束の引数,
    pub 目標: 位置<ワールド>,
}

pub(super) struct 剛体の目標拘束のバッチ {
    拘束一覧: Vec<添字付き剛体の目標拘束>,
    係数一覧: Vec<剛体の目標拘束の一刻みの係数>,
    乗数一覧: Vec<ラグランジュ乗数>,
}

impl 剛体の目標拘束のバッチ {
    pub(super) fn 生成する(
        拘束一覧: Vec<添字付き剛体の目標拘束>, 刻み幅: 刻み幅
    ) -> Result<Self, 剛体の参照計算エラー> {
        let mut 係数一覧 = Vec::with_capacity(拘束一覧.len());
        for 拘束 in &拘束一覧 {
            係数一覧.push(拘束.引数.刻み幅で解く係数を導く(刻み幅)?);
        }
        Ok(Self {
            乗数一覧: vec![ラグランジュ乗数::零(); 拘束一覧.len()],
            拘束一覧,
            係数一覧,
        })
    }

    pub(super) fn 乗数を零にする(&mut self) {
        self.乗数一覧.iter_mut().for_each(|乗数| *乗数 = ラグランジュ乗数::零());
    }

    pub(super) fn 乗数一覧(&self) -> &[ラグランジュ乗数] {
        &self.乗数一覧
    }

    /// 並びの順に射影し、補正を直ちに剛体の予測へ足す(逐次)。
    pub(super) fn 一回反復する(&mut self, 剛体一覧: &mut [細分の中の剛体]) {
        for (添字, 拘束) in self.拘束一覧.iter().enumerate() {
            let 剛体 = &mut 剛体一覧[拘束.剛体.配列添字()];
            if let 剛体の目標拘束の一回の射影の結果::補正した {
                補正,
                更新後のラグランジュ乗数,
            } = self.係数一覧[添字].一回射影する(&剛体.参加者(), 拘束.目標, self.乗数一覧[添字])
            {
                剛体.補正を適用する(&補正);
                self.乗数一覧[添字] = 更新後のラグランジュ乗数;
            }
        }
    }
}
