//! 拘束グラフのGPU表現(判断9): 数学の型と分離した明示の符号化でバイト列を作る。
//! 静的な引数(点の初期状態・拘束の引数・隣接表)だけをここが符号化し、毎刻み書き換える状態(ラグランジュ乗数・
//! 前の位置・方式ごとの作業域)はGPU側が零から確保するためバイト列を持たない。
//! 点は16バイト(位置xyz + 逆質量)、拘束は16バイト(a添字 + b添字 + 静止長 + コンプライアンス)、隣接表は区間の開始が
//! 4バイト×(点の数 + 1)、項目が4バイト×(拘束の数の2倍)である。blitz_renderのslang(バインディング表は`shaders/xpbd_step.slang`の冒頭)がこの並びで読む。

use crate::constraint_graph::{拘束グラフ, 点ごとの拘束の隣接表};

pub const 点1件のバイト数: usize = 16;
pub const 拘束1件のバイト数: usize = 16;

pub fn 点の状態バイト列にする(グラフ: &拘束グラフ) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(グラフ.点の数() * 点1件のバイト数);
    for 点 in グラフ.点一覧() {
        for 成分 in [点.位置.x(), 点.位置.y(), 点.位置.z()] {
            バイト列.extend_from_slice(&成分.値().to_le_bytes());
        }
        バイト列.extend_from_slice(&点.逆質量.値().to_le_bytes());
    }
    バイト列
}

pub fn 拘束の引数バイト列にする(グラフ: &拘束グラフ) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(グラフ.拘束の数() * 拘束1件のバイト数);
    for 拘束 in グラフ.拘束一覧() {
        バイト列.extend_from_slice(&拘束.a.値().to_le_bytes());
        バイト列.extend_from_slice(&拘束.b.値().to_le_bytes());
        バイト列.extend_from_slice(&拘束.引数.静止長.値().値().to_le_bytes());
        バイト列.extend_from_slice(&拘束.引数.コンプライアンス.値().to_le_bytes());
    }
    バイト列
}

/// 隣接表の区間の開始の並び。長さは点の数 + 1である。
pub fn 隣接の区間バイト列にする(隣接表: &点ごとの拘束の隣接表) -> Vec<u8> {
    語の並びをバイト列にする(隣接表.区間の開始一覧().iter().copied())
}

/// 隣接表の項目の並び。1語が拘束添字の2倍に側を足した値である(`隣接の項目::一語へ符号化する`)。
pub fn 隣接の項目バイト列にする(隣接表: &点ごとの拘束の隣接表) -> Vec<u8> {
    語の並びをバイト列にする(隣接表.項目一覧().iter().map(|項目| 項目.一語へ符号化する()))
}

fn 語の並びをバイト列にする(語一覧: impl Iterator<Item = u32>) -> Vec<u8> {
    let mut バイト列 = Vec::new();
    for 語 in 語一覧 {
        バイト列.extend_from_slice(&語.to_le_bytes());
    }
    バイト列
}
