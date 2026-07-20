//! ビルド時に埋め込まれた布シミュレーションのシェーダー(判断54)。
//! ファイル名は`build_support::cloth_spirv_compile`の出力名と一致させる。
//! ホットリロードは粒子・スキニングと同様に非対応(ビルド時コンパイルのみ)。

use blitz_render::{コンピュートシェーダー, シェーダー一式, 布シェーダー一式};

use crate::error::起動エラー;

const 介入SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_intervention.spv"));
const 積分SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_integrate.spv"));
const アタッチSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_attach.spv"));
const 拘束SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_constraint.spv"));
const ハッシュ消去SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_hash_clear.spv"));
const ハッシュ格納SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_hash_store.spv"));
const 分離SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_separate.spv"));
const 仕上げSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_finish.spv"));
const 頂点生成SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_vertex_gen.spv"));
const 描画頂点SPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_draw_vertex.spv"));
const 描画フラグメントSPIRV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cloth_draw_fragment.spv"));

pub(crate) fn 埋め込み布シェーダーを生成する() -> Result<布シェーダー一式, 起動エラー> {
    Ok(布シェーダー一式 {
        介入: コンピュートシェーダー::生成する(介入SPIRV.to_vec())?,
        積分: コンピュートシェーダー::生成する(積分SPIRV.to_vec())?,
        アタッチ: コンピュートシェーダー::生成する(アタッチSPIRV.to_vec())?,
        拘束: コンピュートシェーダー::生成する(拘束SPIRV.to_vec())?,
        ハッシュ消去: コンピュートシェーダー::生成する(ハッシュ消去SPIRV.to_vec())?,
        ハッシュ格納: コンピュートシェーダー::生成する(ハッシュ格納SPIRV.to_vec())?,
        分離: コンピュートシェーダー::生成する(分離SPIRV.to_vec())?,
        仕上げ: コンピュートシェーダー::生成する(仕上げSPIRV.to_vec())?,
        頂点生成: コンピュートシェーダー::生成する(頂点生成SPIRV.to_vec())?,
        描画: シェーダー一式::生成する(描画頂点SPIRV.to_vec(), 描画フラグメントSPIRV.to_vec())?,
    })
}
