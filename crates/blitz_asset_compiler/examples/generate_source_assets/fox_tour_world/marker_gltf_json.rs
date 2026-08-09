//! 目印の柱のglTF文書を組み立てる工程。受け取るのはバイナリのファイル名、返すのは1メッシュ1プリミティブの文書である。
//! bufferViewsの開始位置と長さは`marker_geometry`が公開する区間のバイト長から導いており、書き出しの順と手で合わせない。
//!
//! ベースカラーを橙にするのは、この世界の地面が草地の緑であり、その補色に近い橙なら目印が背景と混ざらないためである。

use super::marker_geometry::{
    インデックスのバイト長, 位置のバイト長, 半幅メートル, 添字数, 頂点数, 高さメートル
};

pub(super) fn 文書(バイナリファイル名: &str) -> String {
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
    {{ "bufferView": 0, "componentType": 5126, "count": {頂点数}, "type": "VEC3", "min": [{左}, 0.0, {左}], "max": [{右}, {高さメートル}, {右}] }},
    {{ "bufferView": 1, "componentType": 5126, "count": {頂点数}, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5123, "count": {添字数}, "type": "SCALAR" }}
  ],
  "materials": [
    {{ "pbrMetallicRoughness": {{ "baseColorFactor": [0.95, 0.45, 0.08, 1.0], "metallicFactor": 0.0, "roughnessFactor": 0.5 }} }}
  ],
  "meshes": [ {{ "name": "目的地の目印の柱", "primitives": [
    {{ "attributes": {{ "POSITION": 0, "NORMAL": 1 }}, "indices": 2, "material": 0 }}
  ] }} ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}
"#,
        左 = -半幅メートル,
        右 = 半幅メートル,
    )
}
