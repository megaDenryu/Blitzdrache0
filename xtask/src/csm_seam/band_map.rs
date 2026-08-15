//! 距離区分の可視化画像から「その画素がどの距離区分を参照したか」と「影の中か」を読み取る工程。
//! 受け取るのは可視化の読み戻し画像、返すのは画素ごとの距離区分と影の地図である。
//!
//! 可視化の色は`shaders/cascade_shadow.slang`の`cascadeDebugColor`が出す。距離区分ごとに使うチャンネルの組が違い、
//! 影の中では同じ色が暗い段へ落ちる。中間の値を作らないため、バイト値の一致だけで読み取れる。
//! 注意: 距離区分数4とチャンネルの組はシェーダー側と二重に持つ。xtaskはエンジンのクレートへ依存しないためである。

use crate::acceptance::{画素の番号, 読み戻し画像};

/// 距離区分ごとのチャンネルの組。`cascadeDebugColor`の`bandColor`と一致させる。
const 距離区分別のチャンネル: [[bool; 3]; 4] = [[true, false, false], [false, true, false], [false, false, true], [true, true, false]];
/// 影の外と中の段。`bandColor`へ掛ける1.0と0.25をsRGBで符号化した値である。
const 明るい段: u8 = 255;
const 暗い段: u8 = 137;
/// 読み取りの許容差。読み戻しの量子化ゆらぎだけを吸収する幅にする。
const 許容差: u8 = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct 距離区分の画素 {
    pub(super) 距離区分: usize,
    pub(super) 影の中: bool,
}

pub(super) struct 距離区分の地図 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    画素一覧: Vec<Option<距離区分の画素>>,
}

impl 距離区分の地図 {
    pub(super) fn 読み取る(可視化: &読み戻し画像) -> Self {
        let 画素一覧 = (0..可視化.画素数())
            .map(|添字| 判別する(可視化.番号の画素(画素の番号::生成する(添字))))
            .collect();
        Self {
            幅: 可視化.幅().画素数(),
            高さ: 可視化.高さ().画素数(),
            画素一覧,
        }
    }

    pub(super) fn 画素(&self, x: usize, y: usize) -> Option<距離区分の画素> {
        self.画素一覧[y * self.幅 + x]
    }

    pub(super) fn 添字(&self, x: usize, y: usize) -> usize {
        y * self.幅 + x
    }

    /// 距離区分ごとの「影の判定を受けた画素数」。影の中かどうかを問わず、その距離区分を参照した画素を数える。
    /// 影の画素数と別に要るのは、遠い距離区分が受光面を覆っていても自己遮蔽の影がほとんど出ない構図があり、
    /// 「その距離区分が画面に現れているか」と「そこに影があるか」が別の問いだからである。
    pub(super) fn 距離区分別の受光画素数(&self) -> [usize; 4] {
        let mut 数 = [0usize; 4];
        for 画素 in self.画素一覧.iter().flatten() {
            数[画素.距離区分] += 1;
        }
        数
    }

    /// 距離区分ごとの「影の中の画素数」。どの距離区分にも影が出ていることをこの数で見る。
    pub(super) fn 距離区分別の影画素数(&self) -> [usize; 4] {
        let mut 数 = [0usize; 4];
        for 画素 in self.画素一覧.iter().flatten() {
            if 画素.影の中 {
                数[画素.距離区分] += 1;
            }
        }
        数
    }
}

fn 判別する(色: [u8; 3]) -> Option<距離区分の画素> {
    for (距離区分, チャンネル) in 距離区分別のチャンネル.iter().enumerate() {
        for (段, 影の中) in [(明るい段, false), (暗い段, true)] {
            let 期待 = チャンネル.map(|使う| if 使う { 段 } else { 0 });
            if (0..3).all(|軸| 色[軸].abs_diff(期待[軸]) <= 許容差) {
                return Some(距離区分の画素 { 距離区分, 影の中 });
            }
        }
    }
    None
}
