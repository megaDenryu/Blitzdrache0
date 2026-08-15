//! 影の欠落計器の診断画像1枚の読み取り。受け取るのは読み戻し画像、返すのは画素ごとの影可視度と受光距離帯である。
//!
//! 影の欠落の検収と、遠景の検収の影の検査点の2つがこの符号化を読む。読む側が2つになったため、
//! どちらかの入口の内側でなくxtaskの共通の語彙として置く。
//!
//! 符号化は`shaders/pixel_diagnostic.slang`の`shadowLossDiagnosticColor`が作る。赤が影可視度、緑が受光距離帯、青が幾何の印である。
//! 出力は交換チェーンのsRGB符号化を通るため、ここで同じ伝達関数の逆をかけて線形へ戻す。
//! 注意: 帯の刻みと帯の数はエンジンのクレートを参照せずここに写しを持つ(xtaskは他クレートへ依存しない規律)。
//! 値の正本は`crates/blitz_render/src/cascade/diagnostic.rs`であり、slangの写しとの一致は`cargo xtask conform`が見る。

mod decode;

use decode::判別する;

use crate::acceptance::{画素の番号, 読み戻し画像};

/// 受光距離帯の数。`受光距離帯数`の写しである。
pub(crate) const 帯数: usize = 16;
/// 受光距離帯1つの幅。`受光距離帯の幅メートル`の写しである。
pub(crate) const 帯の幅メートル: f64 = 25.0;
/// 影の中とみなす影可視度の上限。PCFは3x3の平均であるため中間の値が出る。半分より暗い画素を影とする。
const 影とみなす影可視度: f64 = 0.5;

#[derive(Clone, Copy)]
pub(crate) struct 診断画素 {
    pub(crate) 影可視度: f64,
    /// 0から始まる受光距離帯の番号。最後の帯は上限を持たない。
    pub(crate) 受光距離帯: usize,
}

impl 診断画素 {
    pub(crate) fn 影の中か(self) -> bool {
        self.影可視度 < 影とみなす影可視度
    }
}

pub(crate) struct 診断画像 {
    pub(crate) 幅: usize,
    pub(crate) 高さ: usize,
    /// 幾何の印が立っていない画素(背景)は`None`である。
    画素一覧: Vec<Option<診断画素>>,
}

impl 診断画像 {
    pub(crate) fn 読み取る(読み戻し: &読み戻し画像) -> Self {
        Self {
            幅: 読み戻し.幅().画素数(),
            高さ: 読み戻し.高さ().画素数(),
            画素一覧: (0..読み戻し.画素数())
                .map(|添字| 判別する(読み戻し.番号の画素(画素の番号::生成する(添字))))
                .collect(),
        }
    }

    /// 起動を伴わずに読み直したRGBA8の連結から組む。採取済みの絵を後から突き合わせる入口が使う。
    /// 透明度を捨てるのは、この診断が赤・緑・青の3成分だけに意味を載せるためである。
    pub(crate) fn rgba8から読み取る(幅: usize, 高さ: usize, rgba8: &[u8]) -> Self {
        Self {
            幅,
            高さ,
            画素一覧: rgba8.chunks_exact(4).map(|画素| 判別する([画素[0], 画素[1], 画素[2]])).collect(),
        }
    }

    pub(crate) fn 画素(&self, 添字: usize) -> Option<診断画素> {
        self.画素一覧.get(添字).copied().flatten()
    }

    pub(crate) fn 画素数(&self) -> usize {
        self.画素一覧.len()
    }
}
