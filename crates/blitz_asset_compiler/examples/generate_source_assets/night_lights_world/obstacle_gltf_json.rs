//! 遮蔽物1件のglTF文書を組み立てる工程。受け取るのはバイナリのファイル名と直方体の寸法とベースカラー、
//! 返すのは1メッシュ1プリミティブの文書である。
//! bufferViewsの開始位置と長さは`obstacle_shape`が公開する区間のバイト長から導いており、書き出しの順と手で合わせない。
//!
//! テクスチャを1枚も持たせないのは、この遮蔽物が絵に出すのが形と影だけであり、面の模様が判定に1つも効かないためである。

use super::obstacle_shape::{インデックスのバイト長, 位置のバイト長, 添字数, 遮蔽物の直方体の寸法, 頂点数};

pub(super) fn 遮蔽物のgltf文書を組み立てる(
    バイナリファイル名: &str, 寸法: 遮蔽物の直方体の寸法, ベースカラー: [f32; 3]
) -> String {
    let 法線の開始 = 位置のバイト長;
    let インデックスの開始 = 法線の開始 + 位置のバイト長;
    let 全長 = インデックスの開始 + インデックスのバイト長;
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "uri": "{バイナリファイル名}", "byteLength": {全長} }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": {位置のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {法線の開始}, "byteLength": {位置のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {インデックスの開始}, "byteLength": {インデックスのバイト長}, "target": 34963 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": {頂点数}, "type": "VEC3", "min": [{西}, 0.0, {奥}], "max": [{東}, {高さ}, {手前}] }},
    {{ "bufferView": 1, "componentType": 5126, "count": {頂点数}, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5123, "count": {添字数}, "type": "SCALAR" }}
  ],
  "materials": [
    {{ "pbrMetallicRoughness": {{ "baseColorFactor": [{赤}, {緑}, {青}, 1.0], "metallicFactor": 0.0, "roughnessFactor": 0.85 }} }}
  ],
  "meshes": [ {{ "name": "夜の多光源の検収世界の遮蔽物", "primitives": [
    {{ "attributes": {{ "POSITION": 0, "NORMAL": 1 }}, "indices": 2, "material": 0 }}
  ] }} ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}
"#,
        西 = -寸法.東西の半幅メートル,
        東 = 寸法.東西の半幅メートル,
        奥 = -寸法.南北の半幅メートル,
        手前 = 寸法.南北の半幅メートル,
        高さ = 寸法.高さメートル,
        赤 = ベースカラー[0],
        緑 = ベースカラー[1],
        青 = ベースカラー[2],
    )
}
