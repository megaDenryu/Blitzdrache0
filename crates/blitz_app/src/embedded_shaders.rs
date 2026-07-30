//! ビルド時に埋め込まれた初期シェーダー(SPIR-V)。シーンシェーダーはホットリロードで
//! 置き換わるまでの起動直後の1回だけに使う。ファイル名は`build_support`配下の
//! 各コンパイルモジュールの出力名と一致させる。
//! トーンマップシェーダーのホットリロードは粒子と同様に非対応(ビルド時コンパイルのみ)。

use blitz_render::{
    コンピュートシェーダー, シェーダー一式, シェーダー束, 大気LUTシェーダー一式, 粒子シェーダー一式
};

use crate::cli::{空の描画指定, 粒子表示モード};
use crate::error::起動エラー;

const 頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vertex.spv"));
const フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fragment.spv"));

const 粒子コンピュートSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_compute.spv"));
const 粒子頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_vertex.spv"));
const 粒子フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/particle_fragment.spv"));
const 表面流コンピュートSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/surface_flow_compute.spv"));
const 表面流頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/surface_flow_vertex.spv"));
const 表面流フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/surface_flow_fragment.spv"));
const SPHコンピュートSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sph_compute.spv"));
const SPH頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sph_vertex.spv"));
const SPHフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sph_fragment.spv"));

const UI頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui_vertex.spv"));
const UIフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui_fragment.spv"));

const シャドウ頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shadow_vertex.spv"));
const シャドウフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shadow_fragment.spv"));

const トーンマップ頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tonemap_vertex.spv"));
const トーンマップフラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tonemap_fragment.spv"));

const ブルーム縮小側頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_down_vertex.spv"));
const ブルーム前処理SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_prefilter.spv"));
const ブルーム縮小SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_downsample.spv"));
const ブルーム拡大側頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_up_vertex.spv"));
const ブルーム拡大SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bloom_upsample.spv"));

const スキニングSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/skinning_compute.spv"));

const 大気透過率SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_transmittance.spv"));
const 大気多重散乱SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_multiscatter.spv"));
const 大気スカイビューSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_skyview.spv"));
const 大気空中遠近SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/atmosphere_aerial.spv"));

/// 大気LUTのコンピュートシェーダーだけを組み立てる。読み戻し検査はレンダラーを作らないため、束の全部を要らない。
pub(crate) fn 埋め込み大気lutシェーダーを生成する() -> Result<大気LUTシェーダー一式, 起動エラー> {
    Ok(大気LUTシェーダー一式 {
        透過率: コンピュートシェーダー::生成する(大気透過率SPIRV.to_vec())?,
        多重散乱: コンピュートシェーダー::生成する(大気多重散乱SPIRV.to_vec())?,
        スカイビュー: コンピュートシェーダー::生成する(大気スカイビューSPIRV.to_vec())?,
        空中遠近: コンピュートシェーダー::生成する(大気空中遠近SPIRV.to_vec())?,
    })
}

/// レンダラー生成に渡す全シェーダーを埋め込みSPIR-Vから組み立てる(判断38)。
/// 粒子系シェーダーは粒子トイまたは表面流の指定時だけ含める。空パスは起動指定の方式で腕を選ぶ。
pub(crate) fn 埋め込みシェーダー束を生成する(
    表示: 粒子表示モード,
    空の描画: 空の描画指定,
) -> Result<シェーダー束, 起動エラー> {
    let 粒子 = match 表示 {
        粒子表示モード::なし => None,
        粒子表示モード::粒子トイ => Some(粒子シェーダー一式::生成する(
            粒子コンピュートSPIRV.to_vec(),
            粒子頂点SPIRV.to_vec(),
            粒子フラグメントSPIRV.to_vec(),
        )?),
        粒子表示モード::表面流 => Some(粒子シェーダー一式::生成する(
            表面流コンピュートSPIRV.to_vec(),
            表面流頂点SPIRV.to_vec(),
            表面流フラグメントSPIRV.to_vec(),
        )?),
        粒子表示モード::Sph512 | 粒子表示モード::Sph1024 | 粒子表示モード::Sph2048 => Some(
            粒子シェーダー一式::生成する(SPHコンピュートSPIRV.to_vec(), SPH頂点SPIRV.to_vec(), SPHフラグメントSPIRV.to_vec())?,
        ),
    };
    Ok(シェーダー束 {
        シーン: シェーダー一式::生成する(頂点SPIRV.to_vec(), フラグメントSPIRV.to_vec())?,
        シャドウ: シェーダー一式::生成する(シャドウ頂点SPIRV.to_vec(), シャドウフラグメントSPIRV.to_vec())?,
        空: crate::embedded_sky_shaders::埋め込み空シェーダーを生成する(空の描画)?,
        大気lut: 埋め込み大気lutシェーダーを生成する()?,
        トーンマップ: シェーダー一式::生成する(トーンマップ頂点SPIRV.to_vec(), トーンマップフラグメントSPIRV.to_vec())?,
        ブルーム前処理: シェーダー一式::生成する(ブルーム縮小側頂点SPIRV.to_vec(), ブルーム前処理SPIRV.to_vec())?,
        ブルーム縮小: シェーダー一式::生成する(ブルーム縮小側頂点SPIRV.to_vec(), ブルーム縮小SPIRV.to_vec())?,
        ブルーム拡大: シェーダー一式::生成する(ブルーム拡大側頂点SPIRV.to_vec(), ブルーム拡大SPIRV.to_vec())?,
        ui: シェーダー一式::生成する(UI頂点SPIRV.to_vec(), UIフラグメントSPIRV.to_vec())?,
        スキニング: コンピュートシェーダー::生成する(スキニングSPIRV.to_vec())?,
        布: crate::embedded_cloth_shaders::埋め込み布シェーダーを生成する()?,
        粒子,
    })
}
