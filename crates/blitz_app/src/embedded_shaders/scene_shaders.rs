//! シーン描画のシェーダーを埋め込みSPIR-Vから組む工程。触れるのは頂点段1本と、契約と材質変種の組ごとの画素段4本だけである。
//!
//! 5本を1箇所へ寄せるのは、頂点段を4組すべてで共有することがこの組み方の要点だからである。
//! 画素段だけが照明問い合わせのセットの束縛とアルベドの組み立てを変えるため、頂点段を組ごとに焼き直す理由が無い
//! (参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Icの消費式と実装段割り」)。
//!
//! ファイル名は`build_support::spirv_compile`の出力名と一致させる。

use blitz_render::indirect_lighting::{契約と材質変種ごとの画素段, 契約別のシーン描画シェーダー};

use crate::error::起動エラー;

pub(super) const 頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vertex.spv"));
pub(super) const 画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fragment.spv"));
pub(super) const 遠方環境の画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/scene_distant_environment_fragment.spv"));
pub(super) const 地表の層の画素段SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/scene_surface_layer_fragment.spv"));
pub(super) const 地表の層の遠方環境の画素段SPIRV: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/scene_surface_layer_distant_environment_fragment.spv"));

pub(super) fn 組む() -> Result<契約別のシーン描画シェーダー, 起動エラー> {
    Ok(契約別のシーン描画シェーダー::生成する(
        頂点SPIRV.to_vec(),
        契約と材質変種ごとの画素段 {
            定数近似の標準金属粗さpbr: 画素段SPIRV.to_vec(),
            定数近似の地表の層の重ね合わせ: 地表の層の画素段SPIRV.to_vec(),
            遠方環境の標準金属粗さpbr: 遠方環境の画素段SPIRV.to_vec(),
            遠方環境の地表の層の重ね合わせ: 地表の層の遠方環境の画素段SPIRV.to_vec(),
        },
    )?)
}
