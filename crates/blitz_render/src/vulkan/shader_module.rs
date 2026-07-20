//! SPIR-Vバイト列からVkShaderModuleを生成する。グラフィックス・コンピュート・
//! 粒子描画の全パイプライン生成が共有する(vulkanモジュール直下に置く理由)。
//!
//! 注意: `ash::util::read_spv` はマジックナンバーを検証し4バイト単位でu32語列へ変換する。
//! `シェーダー一式`・`粒子シェーダー一式` は生成時に4バイト整列を検証済みのため、
//! ここでの失敗はslangcが不正なSPIR-Vを出力した場合のみ発生しうる。

use std::io::Cursor;

use ash::util::read_spv;
use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) fn 生成する(
    device: &ash::Device,
    spirvバイト列: &[u8],
) -> Result<vk::ShaderModule, レンダラーエラー> {
    let 語列 = read_spv(&mut Cursor::new(spirvバイト列))
        .map_err(|誤り| レンダラーエラー::SPIRV読み込み失敗(誤り.to_string()))?;
    let create_info = vk::ShaderModuleCreateInfo::default().code(&語列);
    // 安全性: create_infoのcodeは直前に構築したVec<u32>を参照し、deviceは生成済みで有効。
    Ok(unsafe { device.create_shader_module(&create_info, None)? })
}
