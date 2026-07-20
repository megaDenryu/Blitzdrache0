//! ピクセル読み戻し結果の画像を表す値オブジェクト。RGBA8並びで保持する。
//!
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断9」。

/// 読み戻した1フレームぶんの画像。幅・高さとRGBA8バイト列を保持する。
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
        let 添字 = u64::from(y).checked_mul(u64::from(行あたりバイト数))?
            + u64::from(x).checked_mul(4)?;
        let 添字 = usize::try_from(添字).ok()?;
        let 成分 = self.rgba.get(添字..添字 + 4)?;
        Some([成分[0], 成分[1], 成分[2], 成分[3]])
    }
}
