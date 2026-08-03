//! シーン描画の契約別シェーダーを埋め込みSPIR-Vから組む工程。触れるのは頂点段1本と契約ごとの画素段2本だけである。
//!
//! 3本を1箇所へ寄せるのは、頂点段を両方の契約で共有することがこの組み方の要点だからである。
//! 画素段だけが照明問い合わせのセットの束縛を変えるため、頂点段を契約ごとに焼き直す理由が無い
//! (参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Icの消費式と実装段割り」)。
//!
//! ファイル名は`build_support::spirv_compile`の出力名と一致させる。

use blitz_render::indirect_lighting::契約別の描画シェーダー;
use blitz_render::シェーダー一式;

use crate::error::起動エラー;

const 頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vertex.spv"));
pub(super) const 画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fragment.spv"));
pub(super) const 遠方環境の画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/scene_distant_environment_fragment.spv"));

pub(super) fn 組む() -> Result<契約別の描画シェーダー, 起動エラー> {
    Ok(契約別の描画シェーダー::生成する(
        シェーダー一式::生成する(頂点SPIRV.to_vec(), 画素段SPIRV.to_vec())?,
        シェーダー一式::生成する(頂点SPIRV.to_vec(), 遠方環境の画素段SPIRV.to_vec())?,
    ))
}
