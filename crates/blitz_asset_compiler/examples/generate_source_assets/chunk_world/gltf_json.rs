//! チャンク1つ分のglTF文書。25チャンクが1つの共有バッファを区間で分け合うため、
//! bufferViewsのオフセットはgeometry::チャンク当たりバイト長の倍数として計算する。
//! マテリアルを持たないためベースカラーはエンジン側の既定テクスチャが使われる。

use blitz_engine::チャンク座標;

use super::geometry::チャンク当たりバイト長;
use super::{チャンク中心を求める, 共有バッファファイル名, 板の半辺};

pub(super) fn 文書を作る(添字: usize, 座標: チャンク座標, 共有バッファ長: usize) -> Result<String, String> {
    let (中心x, 中心z) = チャンク中心を求める(座標)?;
    let 基点 = 添字 * チャンク当たりバイト長;
    let 最小 = format!("[{}, 0.0, {}]", 中心x - 板の半辺, 中心z - 板の半辺);
    let 最大 = format!("[{}, 0.0, {}]", 中心x + 板の半辺, 中心z + 板の半辺);
    Ok(format!(
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
    ))
}
