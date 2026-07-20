//! コンピュート単独のSPIR-Vバイト列を保持する値オブジェクト(判断44のスキニング用)。
//! 検証規則は`シェーダー一式`と同じ(空でない・4バイト整列)ため、エラー型を共用する。

use crate::shader_set::シェーダー一式エラー;

#[derive(Debug, Clone)]
pub struct コンピュートシェーダー {
    spirv: Vec<u8>,
}

impl コンピュートシェーダー {
    /// バイト列が空でなく4の倍数長であることを検証して生成する。
    pub fn 生成する(spirv: Vec<u8>) -> Result<Self, シェーダー一式エラー> {
        if spirv.is_empty() {
            return Err(シェーダー一式エラー::空のバイト列);
        }
        if !spirv.len().is_multiple_of(4) {
            return Err(シェーダー一式エラー::非4バイト整列(spirv.len()));
        }
        Ok(Self { spirv })
    }

    pub(crate) fn コード(&self) -> &[u8] {
        &self.spirv
    }
}
