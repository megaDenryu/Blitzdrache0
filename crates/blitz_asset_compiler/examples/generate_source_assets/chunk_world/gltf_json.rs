//! チャンク1つ分のglTF文書。25チャンクが1つの共有バッファを区間で分け合うため、
//! bufferViewsの開始位置はgeometry::チャンク当たりバイト長の倍数として計算する。
//! マテリアルを持たないためベースカラーはエンジン側の既定テクスチャが使われる。

use super::geometry::チャンク当たりバイト長;
use super::{共有バッファファイル名, 板の半辺, 板の局所中心};

pub(super) fn 文書を作る(添字: usize, 共有バッファ長: usize) -> String {
    let 基点 = 添字 * チャンク当たりバイト長;
    let 端最小 = 板の局所中心 - 板の半辺;
    let 端最大 = 板の局所中心 + 板の半辺;
    let 最小 = format!("[{端最小}, 0.0, {端最小}]");
    let 最大 = format!("[{端最大}, 0.0, {端最大}]");
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [
    {{ "uri": "{共有バッファファイル名}", "byteLength": {共有バッファ長} }}
  ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": {位置区間}, "byteLength": 48, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {法線区間}, "byteLength": 48, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {接線区間}, "byteLength": 64, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {uv区間}, "byteLength": 32, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {インデックス区間}, "byteLength": 12, "target": 34963 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": 4, "type": "VEC3", "min": {最小}, "max": {最大} }},
    {{ "bufferView": 1, "componentType": 5126, "count": 4, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5126, "count": 4, "type": "VEC4" }},
    {{ "bufferView": 3, "componentType": 5126, "count": 4, "type": "VEC2" }},
    {{ "bufferView": 4, "componentType": 5123, "count": 6, "type": "SCALAR" }}
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
"#,
        位置区間 = 基点,
        法線区間 = 基点 + 48,
        接線区間 = 基点 + 96,
        uv区間 = 基点 + 160,
        インデックス区間 = 基点 + 192,
    )
}
