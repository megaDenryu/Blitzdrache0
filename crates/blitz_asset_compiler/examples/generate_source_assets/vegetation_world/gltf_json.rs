//! 植生の原型1体ぶんのglTF文書。bufferViewsのオフセットはgeometryの区間長から計算する。
//! マテリアルを持たないためベースカラーはエンジン側の既定テクスチャ(白)が使われ、番兵背景色との差が最大になる。

use super::geometry::{インデックス区間長, テクスチャ座標区間長, 位置区間長, 接線区間長, 法線区間長};
use super::{共有バッファファイル名, 原型の半辺, 原型の高さ};

pub(super) fn 文書を作る(バッファ長: usize) -> String {
    let 法線区間 = 位置区間長;
    let 接線区間 = 法線区間 + 法線区間長;
    let uv区間 = 接線区間 + 接線区間長;
    let インデックス区間 = uv区間 + テクスチャ座標区間長;
    let 最小 = format!("[{}, 0.0, {}]", -原型の半辺, -原型の半辺);
    let 最大 = format!("[{原型の半辺}, {原型の高さ}, {原型の半辺}]");
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [
    {{ "uri": "{共有バッファファイル名}", "byteLength": {バッファ長} }}
  ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": {位置区間長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {法線区間}, "byteLength": {法線区間長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {接線区間}, "byteLength": {接線区間長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {uv区間}, "byteLength": {テクスチャ座標区間長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {インデックス区間}, "byteLength": {インデックス区間長}, "target": 34963 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": 24, "type": "VEC3", "min": {最小}, "max": {最大} }},
    {{ "bufferView": 1, "componentType": 5126, "count": 24, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5126, "count": 24, "type": "VEC4" }},
    {{ "bufferView": 3, "componentType": 5126, "count": 24, "type": "VEC2" }},
    {{ "bufferView": 4, "componentType": 5123, "count": 36, "type": "SCALAR" }}
  ],
  "meshes": [
    {{
      "primitives": [
        {{
          "attributes": {{ "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 }},
          "indices": 4
        }}
      ]
    }}
  ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}
"#
    )
}
