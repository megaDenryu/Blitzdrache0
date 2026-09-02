//! 点ごとの拘束の隣接表。点から、その点が参加する拘束の添字と側を引く表であり、二段階方式の集める工程と、
//! 同時に更新する方式の緩和係数(接続数)がこれを読む。
//! 区間の開始と項目の2本の配列で持つのは、GPUへそのまま2本のバッファとして写せる形にするためである
//! (点ごとの可変長の一覧を固定幅のスロットへ詰める現在の布の形と違い、接続数が不均一なグラフでも空きを作らない)。

use super::constraint_index::拘束添字;
use super::graph::拘束グラフ;
use super::point_index::点添字;

/// 拘束のどちら側の参加点か。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 隣接の側 {
    A,
    B,
}

/// 隣接表の1項目。GPUでは拘束添字を2倍して側を最下位ビットへ載せた1語で表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 隣接の項目 {
    pub 拘束: 拘束添字,
    pub 側: 隣接の側,
}

impl 隣接の項目 {
    /// GPUの1語への符号化。最下位ビットが側(0=a、1=b)、残りが拘束添字である。
    pub fn 一語へ符号化する(&self) -> u32 {
        let 側 = match self.側 {
            隣接の側::A => 0,
            隣接の側::B => 1,
        };
        self.拘束.値() * 2 + 側
    }
}

/// 点ごとの隣接の項目を、区間の開始の並び(点の数 + 1)と項目の並び(拘束の数の2倍)で持つ表。
#[derive(Debug, Clone, PartialEq)]
pub struct 点ごとの拘束の隣接表 {
    区間の開始一覧: Vec<u32>,
    項目一覧: Vec<隣接の項目>,
}

impl 点ごとの拘束の隣接表 {
    /// 拘束一覧を1度走査して、各点の区間へ拘束添字の昇順に項目を並べる。
    pub fn 生成する(グラフ: &拘束グラフ) -> Self {
        let 接続数 = グラフ.点ごとの接続数();
        let mut 区間の開始一覧 = Vec::with_capacity(接続数.len() + 1);
        let mut 累積 = 0u32;
        for 数 in &接続数 {
            区間の開始一覧.push(累積);
            累積 += 数;
        }
        区間の開始一覧.push(累積);
        let mut 書き込み位置: Vec<usize> = 区間の開始一覧.iter().map(|開始| usize::try_from(*開始).unwrap_or(0)).collect();
        let 空 = 隣接の項目 {
            拘束: 拘束添字::生成する(0),
            側: 隣接の側::A,
        };
        let mut 項目一覧 = vec![空; usize::try_from(累積).unwrap_or(0)];
        for (配列添字, 拘束) in グラフ.拘束一覧().iter().enumerate() {
            let 添字 = 拘束添字::配列添字から生成する(配列添字);
            for (点, 側) in [(拘束.a, 隣接の側::A), (拘束.b, 隣接の側::B)] {
                let 位置 = &mut 書き込み位置[点.配列添字()];
                項目一覧[*位置] = 隣接の項目 { 拘束: 添字, 側 };
                *位置 += 1;
            }
        }
        Self {
            区間の開始一覧, 項目一覧
        }
    }

    pub fn 区間の開始一覧(&self) -> &[u32] {
        &self.区間の開始一覧
    }

    pub fn 項目一覧(&self) -> &[隣接の項目] {
        &self.項目一覧
    }

    /// その点が参加する拘束の項目。
    pub fn 点の項目(&self, 点: 点添字) -> &[隣接の項目] {
        let 開始 = usize::try_from(self.区間の開始一覧[点.配列添字()]).unwrap_or(0);
        let 終わり = usize::try_from(self.区間の開始一覧[点.配列添字() + 1]).unwrap_or(0);
        &self.項目一覧[開始..終わり]
    }

    /// その点の接続数。
    pub fn 接続数(&self, 点: 点添字) -> u32 {
        self.区間の開始一覧[点.配列添字() + 1] - self.区間の開始一覧[点.配列添字()]
    }
}
