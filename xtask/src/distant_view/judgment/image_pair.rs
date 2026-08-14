//! RGBA8とD32の対を同じ寸法の検収画像として読む。

use std::path::Path;

pub(super) struct 検収画像 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    pub(super) 色: Vec<u8>,
    pub(super) 深度: Vec<f32>,
}

impl 検収画像 {
    pub(super) fn 読む(置き場: &Path, 名前: &str) -> Result<Self, String> {
        let 寸法 = std::fs::read_to_string(置き場.join(format!("{名前}_color.size"))).map_err(|誤り| 誤り.to_string())?;
        let mut 数 = 寸法.split_whitespace().map(str::parse::<usize>);
        let 幅 = 数.next().ok_or_else(|| "幅が無い".to_string())?.map_err(|誤り| 誤り.to_string())?;
        let 高さ = 数.next().ok_or_else(|| "高さが無い".to_string())?.map_err(|誤り| 誤り.to_string())?;
        let 色 = std::fs::read(置き場.join(format!("{名前}_color.raw"))).map_err(|誤り| 誤り.to_string())?;
        let 深度バイト = std::fs::read(置き場.join(format!("{名前}_depth.depth32"))).map_err(|誤り| 誤り.to_string())?;
        let 画素数 = 幅.checked_mul(高さ).ok_or_else(|| "画素数が桁あふれした".to_string())?;
        if 色.len() != 画素数 * 4 || 深度バイト.len() != 画素数 * 4 {
            return Err(format!("{名前}の寸法とバイト長が一致しない"));
        }
        let 深度 = 深度バイト
            .chunks_exact(4)
            .map(|値| f32::from_le_bytes([値[0], 値[1], 値[2], 値[3]]))
            .collect();
        Ok(Self { 幅, 高さ, 色, 深度 })
    }

    pub(super) fn 同じ寸法を課す(&self, 相手: &Self) -> Result<(), String> {
        if self.幅 == 相手.幅 && self.高さ == 相手.高さ {
            Ok(())
        } else {
            Err("referenceとcandidateの寸法が違う".to_string())
        }
    }

    pub(super) fn 画素数(&self) -> usize {
        self.幅 * self.高さ
    }
    pub(super) fn 色画素(&self, 添字: usize) -> &[u8] {
        &self.色[添字 * 4..添字 * 4 + 4]
    }
    pub(super) fn 深度画素(&self, 添字: usize) -> [u8; 4] {
        self.深度[添字].to_le_bytes()
    }
    pub(super) fn 非有限深度数(&self) -> usize {
        self.深度.iter().filter(|値| !値.is_finite()).count()
    }
}
