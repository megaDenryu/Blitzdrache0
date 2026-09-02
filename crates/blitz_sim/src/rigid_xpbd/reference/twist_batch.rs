//! 剛体と剛体のねじり拘束の密なバッチ(判断10): 2つの剛体の識別子と静止の相対姿勢と軸とコンプライアンスと一細分内の回転の乗数を同じ添字で持つ。

use super::super::error::剛体の参照計算エラー;
use super::super::rotational_lagrange_multiplier::回転のラグランジュ乗数;
use super::super::twist_constraint::{ねじり拘束の一刻みの係数, ねじり拘束の引数};
use super::super::twist_result::ねじり拘束の一回の射影の結果;
use super::substep_body::細分の中の剛体;
use crate::rigid_body::剛体の識別子;
use crate::xpbd::刻み幅;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 添字付きねじり拘束 {
    pub 剛体a: 剛体の識別子,
    pub 剛体b: 剛体の識別子,
    pub 引数: ねじり拘束の引数,
}

pub(super) struct ねじり拘束のバッチ {
    拘束一覧: Vec<添字付きねじり拘束>,
    係数一覧: Vec<ねじり拘束の一刻みの係数>,
    乗数一覧: Vec<回転のラグランジュ乗数>,
}

impl ねじり拘束のバッチ {
    pub(super) fn 生成する(拘束一覧: Vec<添字付きねじり拘束>, 刻み幅: 刻み幅) -> Result<Self, 剛体の参照計算エラー> {
        let mut 係数一覧 = Vec::with_capacity(拘束一覧.len());
        for 拘束 in &拘束一覧 {
            if 拘束.剛体a == 拘束.剛体b {
                return Err(剛体の参照計算エラー::同じ剛体を結ぶ拘束 { 剛体: 拘束.剛体a });
            }
            係数一覧.push(拘束.引数.刻み幅で解く係数を導く(刻み幅)?);
        }
        Ok(Self {
            乗数一覧: vec![回転のラグランジュ乗数::零(); 拘束一覧.len()],
            拘束一覧,
            係数一覧,
        })
    }

    pub(super) fn 乗数を零にする(&mut self) {
        self.乗数一覧.iter_mut().for_each(|乗数| *乗数 = 回転のラグランジュ乗数::零());
    }

    pub(super) fn 乗数一覧(&self) -> &[回転のラグランジュ乗数] {
        &self.乗数一覧
    }

    // 並びの順に射影し、2つの回転の補正を直ちに各剛体の予測へ足す(逐次)。
    pub(super) fn 一回反復する(&mut self, 剛体一覧: &mut [細分の中の剛体]) {
        for (添字, 拘束) in self.拘束一覧.iter().enumerate() {
            let (a, b) = (拘束.剛体a.配列添字(), 拘束.剛体b.配列添字());
            let 結果 = self.係数一覧[添字].一回射影する(&剛体一覧[a].参加者(), &剛体一覧[b].参加者(), self.乗数一覧[添字]);
            if let ねじり拘束の一回の射影の結果::補正した {
                補正a,
                補正b,
                更新後のラグランジュ乗数,
            } = 結果
            {
                剛体一覧[a].補正を適用する(&補正a);
                剛体一覧[b].補正を適用する(&補正b);
                self.乗数一覧[添字] = 更新後のラグランジュ乗数;
            }
        }
    }
}
