//! ビルド時に埋め込まれた派生表現の3本のSPIR-Vと、その組み立て。
//!
//! 遠方環境の1本と別に持つのは、遠方環境だけを焼く読み戻し検査が派生表現のシェーダーを抱えないためである。
//! ファイル名は`build_support/atmosphere_spirv_compile.rs`の出力名と一致させる。
//! レンダラーへ渡す4本の束を組むのもここが持つ。3本の出どころがここであり、遠方環境の1本を足すだけで足りるためである。

use blitz_render::distant_environment::遠方環境のシェーダー一式;
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

/// レンダラーの束へ渡す4本。照明問い合わせ契約が定数近似でも渡すのは、束の有無をもう1つの一致条件にしないためである。
pub(crate) fn 埋め込み遠方環境シェーダー一式を生成する() -> Result<遠方環境のシェーダー一式, 起動エラー> {
    let 派生 = 埋め込み派生表現シェーダーを生成する()?;
    Ok(遠方環境のシェーダー一式 {
        遠方環境: crate::embedded_distant_environment_shader::埋め込み遠方環境シェーダーを生成する()?,
        拡散照度: 派生.拡散照度,
        鏡面畳込み: 派生.鏡面畳込み,
        反射率積分表: 派生.反射率積分表,
    })
}
