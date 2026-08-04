//! 読み戻した1フレームの画像を表す値オブジェクト。提示画像のRGBA8並びと、明るさの圧縮前のHDR中間画像の単精度並びの2つを持つ。
//!
//! 2つを1つのモジュールへ置くのは、どちらも同じ読み戻しの結末であり、読み戻し対象の別だけで表現が分かれるためである。
//! 片方だけを見せると、8ビットの値から圧縮前の放射輝度を復元しようとする誤りを招く。
//!
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断9」。

/// 読み戻した1フレームぶんの提示画像。幅・高さとRGBA8バイト列を保持する。
#[derive(Debug, Clone)]
pub struct 読み戻し画像 {
    幅: u32,
    高さ: u32,
    rgba: Vec<u8>,
}

impl 読み戻し画像 {
    /// 前提: `rgba` は `幅 * 高さ * 4` バイトのRGBA8並び済みデータであること
    /// （BGRA等からの並び替えは呼び出し元のvulkan層が済ませる）。
    pub(crate) fn 生成する(幅: u32, 高さ: u32, rgba: Vec<u8>) -> Self {
        Self { 幅, 高さ, rgba }
    }

    pub fn 幅(&self) -> u32 {
        self.幅
    }

    pub fn 高さ(&self) -> u32 {
        self.高さ
    }

    /// 指定座標のRGBA8成分を返す。範囲外なら`None`。
    pub fn ピクセル(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.幅 || y >= self.高さ {
            return None;
        }
        let 行あたりバイト数 = self.幅.checked_mul(4)?;
        let 添字 = u64::from(y).checked_mul(u64::from(行あたりバイト数))? + u64::from(x).checked_mul(4)?;
        let 添字 = usize::try_from(添字).ok()?;
        let 成分 = self.rgba.get(添字..添字 + 4)?;
        Some([成分[0], 成分[1], 成分[2], 成分[3]])
    }
}

/// 読み戻した1フレームぶんの、明るさの圧縮を通す前のHDR中間画像。半精度4成分を単精度へ開いた並びで保持する。
/// 保持する値は露出倍率を掛ける前のリニアの値であり、絵の見え方に依らない放射輝度の相対値である。
#[derive(Debug, Clone)]
pub struct HDR読み戻し画像 {
    幅: u32,
    高さ: u32,
    成分: Vec<f32>,
}

impl HDR読み戻し画像 {
    /// 前提: `成分` は `幅 * 高さ * 4` 要素の、行優先でRGBAの順に並んだ単精度の列であること。
    pub(crate) fn 生成する(幅: u32, 高さ: u32, 成分: Vec<f32>) -> Self {
        Self { 幅, 高さ, 成分 }
    }

    pub fn 幅(&self) -> u32 {
        self.幅
    }

    pub fn 高さ(&self) -> u32 {
        self.高さ
    }

    /// 行優先でRGBAの順に並んだ単精度の成分列。書き出し側はこの並びをそのまま外部形式へ写す。
    pub fn 成分列(&self) -> &[f32] {
        &self.成分
    }
}
