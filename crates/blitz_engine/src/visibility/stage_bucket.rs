//! 群1つぶんの判定を、LOD段ごとの連続範囲へ並べ替える工程。受け取るのは個体ごとの(段番号, 可視)の並び、
//! 書き出すのは段ごとに連続し各段の先頭側へ可視個体を寄せたID列と、段ごとの範囲である。
//!
//! 各段の範囲の先頭側へ可視個体を置くのは、シーンパスが範囲の先頭から可視数ぶんを、シャドウパスが同じ範囲の
//! 全体を描くという1本の列で両パスを賄うためである。列を2本に分けると、個体数ぶんのホスト可視メモリと
//! 毎フレームの書き込み量がどちらも2倍になる(参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」)。
//!
//! 作業領域を型が持ち続けるのは、群ごとに列を確保して捨てることを毎フレーム繰り返さないためである。
//! 段ごとに数える4本の列は`tally`が持つ。

mod tally;

use blitz_render::段別描画範囲;

use tally::段別集計;

#[derive(Clone, Copy)]
struct 個体判定 {
    段番号: u8,
    可視: bool,
}

#[derive(Default)]
pub(super) struct 段別並べ替え {
    個体別判定: Vec<個体判定>,
    集計: 段別集計,
}

impl 段別並べ替え {
    pub(super) fn 群を始める(&mut self, 段数: usize) {
        self.個体別判定.clear();
        self.集計.群を始める(段数);
    }

    pub(super) fn 個体を積む(&mut self, 段番号: u8, 可視: bool) {
        self.集計.個体を数える(usize::from(段番号), 可視);
        self.個体別判定.push(個体判定 { 段番号, 可視 });
    }

    pub(super) fn 可視数(&self) -> usize {
        self.個体別判定.iter().filter(|判定| 判定.可視).count()
    }

    pub(super) fn 書き出す(&mut self, id列: &mut Vec<u32>, 段範囲列: &mut Vec<段別描画範囲>) {
        let 基準 = id列.len();
        id列.resize(基準 + self.個体別判定.len(), 0);
        let mut 開始 = 0u32;
        for 段番号 in 0..self.集計.段数() {
            let (可視数, 個体数) = self.集計.段の範囲を据える(段番号, 開始);
            段範囲列.push(段別描画範囲::生成する(開始, 可視数, 個体数));
            開始 += 個体数;
        }
        for (添字, 判定) in self.個体別判定.iter().enumerate() {
            let 位置 = self.集計.書込位置を取り出して進める(usize::from(判定.段番号), 判定.可視);
            let 書込先 = 基準 + 数へ変換する(位置);
            match id列.get_mut(書込先) {
                Some(枠) => *枠 = idへ変換する(添字),
                None => panic!("段別の書込位置が群のID列の内側に収まるという不変条件に違反した: {書込先}"),
            }
        }
    }
}

fn 数へ変換する(値: u32) -> usize {
    usize::try_from(値).unwrap_or_else(|_| panic!("段別の書込位置がusizeに収まらない: {値}"))
}

/// 群可視材料の生成時に個体数がu32へ収まることを確かめてあるため、この変換は必ず成功する。
fn idへ変換する(添字: usize) -> u32 {
    u32::try_from(添字).unwrap_or_else(|_| panic!("群可視材料の個体数がu32に収まるという不変条件に違反した"))
}
