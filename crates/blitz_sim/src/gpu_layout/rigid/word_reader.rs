//! バイト列を1件ずつ語(4バイト)の並びとして読む読み手。長さが1件のバイト数の倍数であることを生成で検査し、件ごとの語の読み出しを添字で行う。
//! 4つのバッファの読み手が共有する工程であり、単精度と符号なし整数の復号をここに閉じる。

use super::error::剛体レイアウトエラー;

pub(super) struct 剛体のバイト列の読み手<'a> {
    バイト列: &'a [u8],
    件のバイト数: usize,
}

impl<'a> 剛体のバイト列の読み手<'a> {
    pub(super) fn 生成する(バイト列: &'a [u8], 件のバイト数: usize) -> Result<Self, 剛体レイアウトエラー> {
        if !バイト列.len().is_multiple_of(件のバイト数) {
            return Err(剛体レイアウトエラー::バイト数が1件の倍数でない {
                バイト数: バイト列.len(),
                一件のバイト数: 件のバイト数,
            });
        }
        Ok(Self {
            バイト列, 件のバイト数
        })
    }

    pub(super) fn 件数(&self) -> usize {
        self.バイト列.len() / self.件のバイト数
    }

    fn 語のバイト(&self, 件: usize, 語番号: usize) -> [u8; 4] {
        let 開始 = 件 * self.件のバイト数 + 語番号 * 4;
        [
            self.バイト列[開始],
            self.バイト列[開始 + 1],
            self.バイト列[開始 + 2],
            self.バイト列[開始 + 3],
        ]
    }

    pub(super) fn 単精度(&self, 件: usize, 語番号: usize) -> f32 {
        f32::from_le_bytes(self.語のバイト(件, 語番号))
    }

    pub(super) fn 語(&self, 件: usize, 語番号: usize) -> u32 {
        u32::from_le_bytes(self.語のバイト(件, 語番号))
    }

    pub(super) fn 単精度3つ(&self, 件: usize, 開始の語番号: usize) -> [f32; 3] {
        [
            self.単精度(件, 開始の語番号),
            self.単精度(件, 開始の語番号 + 1),
            self.単精度(件, 開始の語番号 + 2),
        ]
    }

    pub(super) fn 単精度4つ(&self, 件: usize, 開始の語番号: usize) -> [f32; 4] {
        [
            self.単精度(件, 開始の語番号),
            self.単精度(件, 開始の語番号 + 1),
            self.単精度(件, 開始の語番号 + 2),
            self.単精度(件, 開始の語番号 + 3),
        ]
    }
}
