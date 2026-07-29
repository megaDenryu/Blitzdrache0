//! プリコンピュートLUTを何テクセルで焼くかの品質方針。

use super::大気数学エラー;

/// 透過率LUTの既定の幅と高さ、多重散乱LUTの既定の一辺。
/// 参照: Sebastien Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique" (EGSR 2020)の実装の解像度。
const 既定の透過率の幅: u32 = 256;
const 既定の透過率の高さ: u32 = 64;
const 既定の多重散乱の一辺: u32 = 32;

/// 各LUTのテクセル数。資源の確保・ディスパッチのワークグループ数・テクセル座標の写像がすべてこの1つの値を読むため、
/// シェーダー側に解像度の定数を持たせない(シェーダーは書き込み先の寸法を実行時に問い合わせる)。
/// 不変条件: どの辺も2以上である。多重散乱LUTの写像が辺の長さから1を引いた値で割るため、1では刻みが定義できない。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 大気LUT解像度 {
    透過率の幅: u32,
    透過率の高さ: u32,
    多重散乱の一辺: u32,
}

impl 大気LUT解像度 {
    pub fn 生成する(透過率の幅: u32, 透過率の高さ: u32, 多重散乱の一辺: u32) -> Result<Self, 大気数学エラー> {
        for (項目, 値) in [
            ("透過率LUTの幅", 透過率の幅),
            ("透過率LUTの高さ", 透過率の高さ),
            ("多重散乱LUTの一辺", 多重散乱の一辺),
        ] {
            if 値 < 2 {
                return Err(大気数学エラー::整数値域外(項目, 値));
            }
        }
        Ok(Self {
            透過率の幅,
            透過率の高さ,
            多重散乱の一辺,
        })
    }

    pub fn 既定値() -> Self {
        Self {
            透過率の幅: 既定の透過率の幅,
            透過率の高さ: 既定の透過率の高さ,
            多重散乱の一辺: 既定の多重散乱の一辺,
        }
    }

    pub fn 透過率の幅(&self) -> u32 {
        self.透過率の幅
    }

    pub fn 透過率の高さ(&self) -> u32 {
        self.透過率の高さ
    }

    pub fn 多重散乱の一辺(&self) -> u32 {
        self.多重散乱の一辺
    }

    pub fn 透過率のテクセル数(&self) -> usize {
        テクセル数(self.透過率の幅, self.透過率の高さ)
    }

    pub fn 多重散乱のテクセル数(&self) -> usize {
        テクセル数(self.多重散乱の一辺, self.多重散乱の一辺)
    }
}

fn テクセル数(幅: u32, 高さ: u32) -> usize {
    let 幅 = usize::try_from(幅).unwrap_or_else(|_| panic!("LUTの幅{幅}がusizeに収まらない"));
    let 高さ = usize::try_from(高さ).unwrap_or_else(|_| panic!("LUTの高さ{高さ}がusizeに収まらない"));
    幅 * 高さ
}
