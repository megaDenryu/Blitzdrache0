//! 粒子の族(粒子トイ・表面流・SPH)の埋め込みSPIR-Vと、表示モードから1組を選ぶ工程。
//! 3つの族をここへまとめるのは、どれも同じ`粒子シェーダー一式`の形を持ち、起動指定が3つのうちの1つを選ぶ
//! という1つの規約でだけ結ばれているためである。選ばない指定では1組も作らない。

use blitz_render::粒子シェーダー一式;

use crate::cli::粒子表示モード;
use crate::error::起動エラー;

const 粒子コンピュートSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_compute.spv"));
const 粒子頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_vertex.spv"));
const 粒子画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_fragment.spv"));
const 表面流コンピュートSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/surface_flow_compute.spv"));
const 表面流頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/surface_flow_vertex.spv"));
const 表面流画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/surface_flow_fragment.spv"));
const SPHコンピュートSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sph_compute.spv"));
const SPH頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sph_vertex.spv"));
const SPH画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sph_fragment.spv"));

/// 表示モードが選んだ1組。`なし`では1組も作らない。
pub(super) fn 表示モードから選ぶ(表示: 粒子表示モード) -> Result<Option<粒子シェーダー一式>, 起動エラー> {
    let 一式 = match 表示 {
        粒子表示モード::なし => None,
        粒子表示モード::粒子トイ => Some(粒子シェーダー一式::生成する(
            粒子コンピュートSPIRV.to_vec(),
            粒子頂点SPIRV.to_vec(),
            粒子画素段SPIRV.to_vec(),
        )?),
        粒子表示モード::表面流 => Some(粒子シェーダー一式::生成する(
            表面流コンピュートSPIRV.to_vec(),
            表面流頂点SPIRV.to_vec(),
            表面流画素段SPIRV.to_vec(),
        )?),
        粒子表示モード::Sph512 | 粒子表示モード::Sph1024 | 粒子表示モード::Sph2048 => Some(
            粒子シェーダー一式::生成する(SPHコンピュートSPIRV.to_vec(), SPH頂点SPIRV.to_vec(), SPH画素段SPIRV.to_vec())?,
        ),
    };
    Ok(一式)
}
