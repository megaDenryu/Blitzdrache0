//! ビルド時に埋め込まれた派生表現の3本のSPIR-Vと、その組み立て。
//!
//! 遠方環境の1本と別に持つのは、遠方環境だけを焼く読み戻し検査が派生表現のシェーダーを抱えないためである。
//! ファイル名は`build_support/atmosphere_spirv_compile.rs`の出力名と一致させる。

use blitz_render::コンピュートシェーダー;

use crate::error::起動エラー;

const 拡散照度SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/diffuse_irradiance.spv"));
const 鏡面畳込みSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/specular_prefilter.spv"));
const 反射率積分表SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/brdf_integration.spv"));

/// 3本をまとめて所有する。読み戻し検査は3本すべてを同時に要る。
pub(crate) struct 派生表現のシェーダー三点 {
    pub(crate) 拡散照度: コンピュートシェーダー,
    pub(crate) 鏡面畳込み: コンピュートシェーダー,
    pub(crate) 反射率積分表: コンピュートシェーダー,
}

pub(crate) fn 埋め込み派生表現シェーダーを生成する() -> Result<派生表現のシェーダー三点, 起動エラー> {
    Ok(派生表現のシェーダー三点 {
        拡散照度: コンピュートシェーダー::生成する(拡散照度SPIRV.to_vec())?,
        鏡面畳込み: コンピュートシェーダー::生成する(鏡面畳込みSPIRV.to_vec())?,
        反射率積分表: コンピュートシェーダー::生成する(反射率積分表SPIRV.to_vec())?,
    })
}
