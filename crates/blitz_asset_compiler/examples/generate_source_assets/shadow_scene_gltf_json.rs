//! shadow_scene.gltfのJSON本体。bufferViewsの開始位置・長さは
//! shadow_scene_geometry::バッファバイト列を作る の書き出し順
//! (床: 位置48B→法線48B→接線64B→UV32B→インデックス12B、遮蔽: 同順)へ
//! 手動で一致させている(判断37)。

pub(super) const 文書JSON: &str = r#"{
  "asset": { "version": "2.0" },
  "buffers": [
    { "uri": "shadow_scene.bin", "byteLength": 408 }
  ],
  "bufferViews": [
    { "buffer": 0, "byteOffset": 0, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 48, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 96, "byteLength": 64, "target": 34962 },
    { "buffer": 0, "byteOffset": 160, "byteLength": 32, "target": 34962 },
    { "buffer": 0, "byteOffset": 192, "byteLength": 12, "target": 34963 },
    { "buffer": 0, "byteOffset": 204, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 252, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 300, "byteLength": 64, "target": 34962 },
    { "buffer": 0, "byteOffset": 364, "byteLength": 32, "target": 34962 },
    { "buffer": 0, "byteOffset": 396, "byteLength": 12, "target": 34963 }
  ],
  "accessors": [
    { "bufferView": 0, "componentType": 5126, "count": 4, "type": "VEC3", "min": [-2.0, 0.0, -2.0], "max": [2.0, 0.0, 2.0] },
    { "bufferView": 1, "componentType": 5126, "count": 4, "type": "VEC3" },
    { "bufferView": 2, "componentType": 5126, "count": 4, "type": "VEC4" },
    { "bufferView": 3, "componentType": 5126, "count": 4, "type": "VEC2" },
    { "bufferView": 4, "componentType": 5123, "count": 6, "type": "SCALAR" },
    { "bufferView": 5, "componentType": 5126, "count": 4, "type": "VEC3", "min": [-0.5, 1.0, -0.5], "max": [0.5, 1.0, 0.5] },
    { "bufferView": 6, "componentType": 5126, "count": 4, "type": "VEC3" },
    { "bufferView": 7, "componentType": 5126, "count": 4, "type": "VEC4" },
    { "bufferView": 8, "componentType": 5126, "count": 4, "type": "VEC2" },
    { "bufferView": 9, "componentType": 5123, "count": 6, "type": "SCALAR" }
  ],
  "images": [
    { "uri": "shadow_scene_white.png" }
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
          "attributes": { "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 },
          "indices": 4,
          "material": 0
        },
        {
          "attributes": { "POSITION": 5, "NORMAL": 6, "TANGENT": 7, "TEXCOORD_0": 8 },
          "indices": 9,
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
