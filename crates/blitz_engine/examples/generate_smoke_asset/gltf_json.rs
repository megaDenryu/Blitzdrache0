//! quad.gltfのJSON本体。bufferViewsのオフセット・長さはgeometry::バッファバイト列を作る
//! の書き出し順(位置48B→法線48B→UV32B→インデックス12B、計140B)へ手動で一致させている。

pub(super) const 文書JSON: &str = r#"{
  "asset": { "version": "2.0" },
  "buffers": [
    { "uri": "quad.bin", "byteLength": 140 }
  ],
  "bufferViews": [
    { "buffer": 0, "byteOffset": 0, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 48, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 96, "byteLength": 32, "target": 34962 },
    { "buffer": 0, "byteOffset": 128, "byteLength": 12, "target": 34963 }
  ],
  "accessors": [
    { "bufferView": 0, "componentType": 5126, "count": 4, "type": "VEC3", "min": [-0.5, -0.5, 0.0], "max": [0.5, 0.5, 0.0] },
    { "bufferView": 1, "componentType": 5126, "count": 4, "type": "VEC3" },
    { "bufferView": 2, "componentType": 5126, "count": 4, "type": "VEC2" },
    { "bufferView": 3, "componentType": 5123, "count": 6, "type": "SCALAR" }
  ],
  "images": [
    { "uri": "quad_base_color.png" }
  ],
  "textures": [
    { "source": 0 }
  ],
  "materials": [
    { "pbrMetallicRoughness": { "baseColorTexture": { "index": 0 } } }
  ],
  "meshes": [
    {
      "primitives": [
        {
          "attributes": { "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 },
          "indices": 3,
          "material": 0
        }
      ]
    }
  ],
  "nodes": [ { "mesh": 0 } ],
  "scenes": [ { "nodes": [0] } ],
  "scene": 0
}
"#;
