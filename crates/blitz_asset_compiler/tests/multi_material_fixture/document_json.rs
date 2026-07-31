//! 試験材料のglTF文書JSONを組み立てる工程。返すのは1メッシュ2プリミティブ2マテリアルの文書である。
//! バッファビューの位置と長さは`binary`の並びへ手で一致させている。

use super::binary::プリミティブの長さ;

pub fn 文書jsonを作る() -> String {
    let 二枚目 = プリミティブの長さ;
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "byteLength": 208 }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": 36 }},
    {{ "buffer": 0, "byteOffset": 36, "byteLength": 36 }},
    {{ "buffer": 0, "byteOffset": 72, "byteLength": 24 }},
    {{ "buffer": 0, "byteOffset": 96, "byteLength": 6 }},
    {{ "buffer": 0, "byteOffset": {二枚目}, "byteLength": 36 }},
    {{ "buffer": 0, "byteOffset": {}, "byteLength": 36 }},
    {{ "buffer": 0, "byteOffset": {}, "byteLength": 24 }},
    {{ "buffer": 0, "byteOffset": {}, "byteLength": 6 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3", "min": [0.0, 0.0, 0.0], "max": [1.0, 1.0, 0.0] }},
    {{ "bufferView": 1, "componentType": 5126, "count": 3, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5126, "count": 3, "type": "VEC2" }},
    {{ "bufferView": 3, "componentType": 5123, "count": 3, "type": "SCALAR" }},
    {{ "bufferView": 4, "componentType": 5126, "count": 3, "type": "VEC3", "min": [2.0, 0.0, 0.0], "max": [3.0, 1.0, 0.0] }},
    {{ "bufferView": 5, "componentType": 5126, "count": 3, "type": "VEC3" }},
    {{ "bufferView": 6, "componentType": 5126, "count": 3, "type": "VEC2" }},
    {{ "bufferView": 7, "componentType": 5123, "count": 3, "type": "SCALAR" }}
  ],
  "materials": [
    {{ "pbrMetallicRoughness": {{ "baseColorFactor": [0.8, 0.3, 0.2, 1.0], "metallicFactor": 0.5, "roughnessFactor": 0.4 }} }},
    {{ "pbrMetallicRoughness": {{ "baseColorFactor": [0.1, 0.7, 0.9, 1.0], "metallicFactor": 0.25, "roughnessFactor": 0.75 }} }}
  ],
  "meshes": [ {{ "name": "二材質の板", "primitives": [
    {{ "attributes": {{ "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 }}, "indices": 3, "material": 0 }},
    {{ "attributes": {{ "POSITION": 4, "NORMAL": 5, "TEXCOORD_0": 6 }}, "indices": 7, "material": 1 }}
  ] }} ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}"#,
        二枚目 + 36,
        二枚目 + 72,
        二枚目 + 96
    )
}
