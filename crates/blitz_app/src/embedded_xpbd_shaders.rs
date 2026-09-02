//! ビルド時に埋め込まれたXPBDの並列方式の計測のシェーダー。ファイル名は`build_support::xpbd_spirv_compile`の出力名と一致させる。

use blitz_render::xpbd_solver_bench_probe::XPBDシェーダー一式;
use blitz_render::コンピュートシェーダー;

use crate::error::起動エラー;

const 積分SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/xpbd_integrate.spv"));
const 乗数零化SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/xpbd_lambda_clear.spv"));
const 原子加算の拘束SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/xpbd_atomic_constraint.spv"));
const 原子加算の適用SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/xpbd_atomic_apply.spv"));
const 彩色の拘束SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/xpbd_coloring_constraint.spv"));
const 二段階の拘束SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/xpbd_two_stage_constraint.spv"));
const 二段階の集約SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/xpbd_two_stage_gather.spv"));

pub(crate) fn 埋め込みxpbdシェーダーを生成する() -> Result<XPBDシェーダー一式, 起動エラー> {
    Ok(XPBDシェーダー一式 {
        積分: コンピュートシェーダー::生成する(積分SPIRV.to_vec())?,
        乗数零化: コンピュートシェーダー::生成する(乗数零化SPIRV.to_vec())?,
        原子加算の拘束: コンピュートシェーダー::生成する(原子加算の拘束SPIRV.to_vec())?,
        原子加算の適用: コンピュートシェーダー::生成する(原子加算の適用SPIRV.to_vec())?,
        彩色の拘束: コンピュートシェーダー::生成する(彩色の拘束SPIRV.to_vec())?,
        二段階の拘束: コンピュートシェーダー::生成する(二段階の拘束SPIRV.to_vec())?,
        二段階の集約: コンピュートシェーダー::生成する(二段階の集約SPIRV.to_vec())?,
    })
}
