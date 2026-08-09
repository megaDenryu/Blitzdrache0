//! テクスチャ用のGPU境界型。CPU側で確定済みのテクスチャ1枚を、格納形式と段ごとのバイト列として保持する。
//! `blitz_engine::格納済みテクスチャ`からの変換は配線層(blitz_app)が行う(このクレートはblitz_engineを知らない)。
//!
//! 語彙は3つに分かれる。格納形式とそのバイト数の算術は`storage_format`、段の寸法の算術は`level_extent`、
//! 段の並びの検査は`level_sequence_check`が持つ。

mod error;
pub(crate) mod level_extent;
mod level_sequence_check;
mod storage_format;
#[cfg(test)]
mod texture_material_tests;
mod usage;

pub use error::テクスチャ素材エラー;
pub use storage_format::テクスチャ格納形式;
pub use usage::テクスチャ用途;

use level_sequence_check::縮小段の並びを検査する;

/// GPUへアップロードするテクスチャ1枚ぶんのCPU側データ。段0から順に並べた縮小段ごとのバイト列を持ち、
/// 縮小段をGPUで作る格納形式では原寸の1本だけを持つ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct テクスチャ素材 {
    幅: u32,
    高さ: u32,
    格納形式: テクスチャ格納形式,
    段ごとのバイト列: Vec<Vec<u8>>,
    用途: テクスチャ用途,
}

impl テクスチャ素材 {
    /// 段の並びが格納形式の契約を満たすことを検証して生成する。この値を持てた時点で、
    /// 画像の縮小段数もコピー領域の寸法も段のバイト長と噛み合っている。
    pub fn 生成する(
        幅: u32,
        高さ: u32,
        格納形式: テクスチャ格納形式,
        段ごとのバイト列: Vec<Vec<u8>>,
        用途: テクスチャ用途,
    ) -> Result<Self, テクスチャ素材エラー> {
        縮小段の並びを検査する(幅, 高さ, 格納形式, &段ごとのバイト列)?;
        Ok(Self {
            幅,
            高さ,
            格納形式,
            段ごとのバイト列,
            用途,
        })
    }

    /// 非圧縮のRGBA8の原寸1枚から生成する。縮小段はGPUが原寸から作るため、段の列は1本になる。
    pub fn rgba8の原寸から生成する(
        幅: u32,
        高さ: u32,
        rgba8: Vec<u8>,
        用途: テクスチャ用途,
    ) -> Result<Self, テクスチャ素材エラー> {
        Self::生成する(幅, 高さ, テクスチャ格納形式::RGBA8, vec![rgba8], 用途)
    }

    pub(crate) fn 幅(&self) -> u32 {
        self.幅
    }

    pub(crate) fn 高さ(&self) -> u32 {
        self.高さ
    }

    pub(crate) fn 格納形式(&self) -> テクスチャ格納形式 {
        self.格納形式
    }

    pub(crate) fn 段ごとのバイト列(&self) -> &[Vec<u8>] {
        &self.段ごとのバイト列
    }

    /// 生成の検査が段の本数を原寸から作れる本数以下に限っているため、この変換は必ず成功する。
    pub(crate) fn 縮小段数(&self) -> u32 {
        let 本数 = self.段ごとのバイト列.len();
        u32::try_from(本数).unwrap_or_else(|_| panic!("縮小段の本数{本数}が32ビットに収まらない(生成の検査が上限を通した不変条件が破れている)"))
    }

    pub(crate) fn 用途(&self) -> テクスチャ用途 {
        self.用途
    }
}
