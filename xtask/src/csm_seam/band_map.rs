//! 帯の可視化画像から「その画素がどの帯を引いたか」と「影の中か」を読み取る工程。
//! 受け取るのは可視化の実行結果、返すのは画素ごとの帯と影の地図である。
//!
//! 可視化の色は`shaders/cascade_shadow.slang`の`cascadeDebugColor`が出す。帯ごとに使うチャンネルの組が違い、
//! 影の中では同じ色が暗い段へ落ちる。中間の値を作らないため、バイト値の一致だけで読み取れる。
//! 注意: 帯数4とチャンネルの組はシェーダー側と二重に持つ。xtaskはエンジンのクレートへ依存しないためである。

use super::run::実行結果;

/// 帯ごとのチャンネルの組。`cascadeDebugColor`の`bandColor`と一致させる。
const 帯別チャンネル: [[bool; 3]; 4] = [[true, false, false], [false, true, false], [false, false, true], [true, true, false]];
/// 影の外と中の段。`bandColor`へ掛ける1.0と0.25をsRGBで符号化した値である。
const 明るい段: u8 = 255;
const 暗い段: u8 = 137;
/// 読み取りの許容差。読み戻しの量子化ゆらぎだけを吸収する幅にする。
const 許容差: u8 = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct 帯画素 {
    pub(super) 帯: usize,
    pub(super) 影の中: bool,
}

pub(super) struct 帯地図 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    画素一覧: Vec<Option<帯画素>>,
}

impl 帯地図 {
    pub(super) fn 読み取る(可視化: &実行結果) -> Self {
        let 画素一覧 = (0..可視化.幅 * 可視化.高さ).map(|添字| 判別する(可視化.画素(添字))).collect();
        Self {
            幅: 可視化.幅,
            高さ: 可視化.高さ,
            画素一覧,
        }
    }

    pub(super) fn 画素(&self, x: usize, y: usize) -> Option<帯画素> {
        self.画素一覧[y * self.幅 + x]
    }

    pub(super) fn 添字(&self, x: usize, y: usize) -> usize {
        y * self.幅 + x
    }

    /// 帯ごとの「影の中の画素数」。どの帯にも影が出ていることをこの数で見る。
    pub(super) fn 帯別の影画素数(&self) -> [usize; 4] {
        let mut 数 = [0usize; 4];
        for 画素 in self.画素一覧.iter().flatten() {
            if 画素.影の中 {
                数[画素.帯] += 1;
            }
        }
        数
    }
}

fn 判別する(色: [u8; 3]) -> Option<帯画素> {
    for (帯, チャンネル) in 帯別チャンネル.iter().enumerate() {
        for (段, 影の中) in [(明るい段, false), (暗い段, true)] {
            let 期待 = チャンネル.map(|使う| if 使う { 段 } else { 0 });
            if (0..3).all(|軸| 色[軸].abs_diff(期待[軸]) <= 許容差) {
                return Some(帯画素 { 帯, 影の中 });
            }
        }
    }
    None
}
