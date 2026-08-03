//! ビルド時に埋め込まれた初期シェーダー(SPIR-V)。シーンシェーダーはホットリロードで
//! 置き換わるまでの起動直後の1回だけに使う。ファイル名は`build_support`配下の
//! 各コンパイルモジュールの出力名と一致させる。
//! 明るさの圧縮シェーダーのホットリロードは粒子と同様に非対応(ビルド時コンパイルのみ)。埋め込んだSPIR-Vそのものへ掛ける検査(材質テクスチャ表への参照の非一様の装飾)は`nonuniform_tests`が持ち、その読み解きは`spirv_module`と`nonuniform_scan`にある。

#[cfg(test)]
mod nonuniform_tests;
mod scene_shaders;

use blitz_render::{
    コンピュートシェーダー, シェーダー一式, シェーダー束, 大気のベイク済み画像のシェーダー一式, 粒子シェーダー一式
};

use crate::cli::{空中遠近合成指定, 粒子表示モード};
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

const UI頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui_vertex.spv"));
const UI画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui_fragment.spv"));

const シャドウ頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shadow_vertex.spv"));
const シャドウ画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shadow_fragment.spv"));

const 明るさの圧縮頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tonemap_vertex.spv"));
const 明るさの圧縮画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tonemap_fragment.spv"));

const 光のにじみ縮小側頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_down_vertex.spv"));
const 光のにじみ前処理SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_prefilter.spv"));
const 光のにじみ縮小SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_downsample.spv"));
const 光のにじみ拡大側頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_up_vertex.spv"));
const 光のにじみ拡大SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_upsample.spv"));

const スキニングSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/skinning_compute.spv"));

const 大気透過率SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_transmittance.spv"));
const 大気多重散乱SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_multiscatter.spv"));
const 大気スカイビューSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_skyview.spv"));
const 大気空中遠近SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_aerial.spv"));

/// 大気のベイク済み画像のコンピュートシェーダーだけを組み立てる。読み戻し検査はレンダラーを作らないため、束の全部を要らない。
pub(crate) fn 埋め込み大気のベイク済み画像シェーダーを生成する() -> Result<大気のベイク済み画像のシェーダー一式, 起動エラー> {
    Ok(大気のベイク済み画像のシェーダー一式 {
        透過率: コンピュートシェーダー::生成する(大気透過率SPIRV.to_vec())?,
        多重散乱: コンピュートシェーダー::生成する(大気多重散乱SPIRV.to_vec())?,
        スカイビュー: コンピュートシェーダー::生成する(大気スカイビューSPIRV.to_vec())?,
        空中遠近: コンピュートシェーダー::生成する(大気空中遠近SPIRV.to_vec())?,
    })
}

/// レンダラー生成に渡す全シェーダーを埋め込みSPIR-Vから組み立てる(判断38)。
/// 粒子系シェーダーは粒子トイまたは表面流の指定時だけ含める。空パスは起動指定`--no-aerial-composite`の
/// 有無で空中遠近合成シェーダーの有無を選ぶ。
pub(crate) fn 埋め込みシェーダー束を生成する(
    表示: 粒子表示モード,
    空中遠近合成: 空中遠近合成指定,
) -> Result<シェーダー束, 起動エラー> {
    let 粒子 = match 表示 {
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
    Ok(シェーダー束 {
        シーン: scene_shaders::組む()?,
        シャドウ: シェーダー一式::生成する(シャドウ頂点SPIRV.to_vec(), シャドウ画素段SPIRV.to_vec())?,
        空: crate::embedded_sky_shaders::埋め込み空シェーダーを生成する(空中遠近合成)?,
        大気のベイク済み画像: 埋め込み大気のベイク済み画像シェーダーを生成する()?,
        遠方環境: crate::embedded_derived_environment_shaders::埋め込み遠方環境シェーダー一式を生成する()?,
        明るさの圧縮: シェーダー一式::生成する(明るさの圧縮頂点SPIRV.to_vec(), 明るさの圧縮画素段SPIRV.to_vec())?,
        光のにじみ前処理: シェーダー一式::生成する(光のにじみ縮小側頂点SPIRV.to_vec(), 光のにじみ前処理SPIRV.to_vec())?,
        光のにじみ縮小: シェーダー一式::生成する(光のにじみ縮小側頂点SPIRV.to_vec(), 光のにじみ縮小SPIRV.to_vec())?,
        光のにじみ拡大: シェーダー一式::生成する(光のにじみ拡大側頂点SPIRV.to_vec(), 光のにじみ拡大SPIRV.to_vec())?,
        ui: シェーダー一式::生成する(UI頂点SPIRV.to_vec(), UI画素段SPIRV.to_vec())?,
        スキニング: コンピュートシェーダー::生成する(スキニングSPIRV.to_vec())?,
        布: crate::embedded_cloth_shaders::埋め込み布シェーダーを生成する()?,
        粒子,
    })
}
