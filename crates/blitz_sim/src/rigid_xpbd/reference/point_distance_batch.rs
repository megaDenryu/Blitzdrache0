//! 剛体と点の距離拘束の密なバッチ(判断10): 点の添字と剛体の識別子と引数と一細分内の乗数を同じ添字で持つ。点自由度と姿勢自由度が同じ拘束へ参加するバッチである。

use super::super::error::剛体の参照計算エラー;
use super::super::point_body_distance_constraint::{点と剛体の距離拘束の一刻みの係数, 点と剛体の距離拘束の引数};
use super::super::point_body_distance_result::点と剛体の距離拘束の一回の射影の結果;
use super::point_state::点の細分の状態;
use super::substep_body::細分の中の剛体;
use crate::constraint_graph::点添字;
use crate::rigid_body::剛体の識別子;
use crate::xpbd::{ラグランジュ乗数, 刻み幅};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 添字付き点と剛体の距離拘束 {
    pub 点: 点添字,
    pub 剛体: 剛体の識別子,
    pub 引数: 点と剛体の距離拘束の引数,
}

pub(super) struct 点と剛体の距離拘束のバッチ {
    拘束一覧: Vec<添字付き点と剛体の距離拘束>,
    係数一覧: Vec<点と剛体の距離拘束の一刻みの係数>,
    乗数一覧: Vec<ラグランジュ乗数>,
}

impl 点と剛体の距離拘束のバッチ {
    pub(super) fn 生成する(
        拘束一覧: Vec<添字付き点と剛体の距離拘束>,
        点の数: usize,
        刻み幅: 刻み幅,
    ) -> Result<Self, 剛体の参照計算エラー> {
        let mut 係数一覧 = Vec::with_capacity(拘束一覧.len());
        for 拘束 in &拘束一覧 {
            if 拘束.点.配列添字() >= 点の数 {
                return Err(剛体の参照計算エラー::点添字が範囲外 {
                    添字: 拘束.点, 点の数
                });
            }
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

    /// 並びの順に射影し、点の補正を位置へ、剛体の補正を予測へ直ちに足す(逐次)。
    pub(super) fn 一回反復する(&mut self, 剛体一覧: &mut [細分の中の剛体], 点一覧: &mut [点の細分の状態]) {
        for (添字, 拘束) in self.拘束一覧.iter().enumerate() {
            let 剛体 = &mut 剛体一覧[拘束.剛体.配列添字()];
            let 点 = &mut 点一覧[拘束.点.配列添字()];
            let 結果 = self.係数一覧[添字].一回射影する(点.参加点(), &剛体.参加者(), self.乗数一覧[添字]);
            if let 点と剛体の距離拘束の一回の射影の結果::補正した {
                点の補正,
                剛体の補正,
                更新後のラグランジュ乗数,
            } = 結果
            {
                点.補正を足す(点の補正);
                剛体.補正を適用する(&剛体の補正);
                self.乗数一覧[添字] = 更新後のラグランジュ乗数;
            }
        }
    }
}
