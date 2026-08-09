//! ブロック圧縮の対照シーンのglTF文書。左の板が滑らかなグラデーション、右の板が決定的な雑音をベースカラーに持つ。
//! bufferViewsの位置と長さは`geometry`の書き出し順(位置96B→法線96B→接線128B→UV64B→インデックス24B、計408B)へ手で一致させている。
//!
//! 2つの素材を1つのシーンへ並べるのは、同じ露出と同じ光の下で2つの誤差の出方を1枚の絵で比べるためである。
//! 別々のシーンにすると、絵の差が素材の違いによるのか描画の条件の違いによるのかを読み分けられない。

use super::geometry;

/// 金属度0・粗さ1にするのは、鏡面反射がベースカラーの誤差の上へ光源の像を重ねるのを避けるためである。
/// 拡散反射だけが残ると、画素の差はベースカラーの差にほぼ比例する。
fn 材質(テクスチャ番号: u32) -> String {
    format!(
        r#"{{ "pbrMetallicRoughness": {{ "baseColorTexture": {{ "index": {テクスチャ番号} }}, "metallicFactor": 0.0, "roughnessFactor": 1.0 }} }}"#
    )
}

pub(super) fn 文書を作る(
    バッファファイル名: &str, グラデーション画像ファイル名: &str, 雑音画像ファイル名: &str
) -> String {
    let (最小, 最大) = geometry::位置の範囲を返す();
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "uri": "{バッファファイル名}", "byteLength": 408 }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": 96, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 96, "byteLength": 96, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 192, "byteLength": 128, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 320, "byteLength": 64, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 384, "byteLength": 24, "target": 34963 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": 8, "type": "VEC3", "min": [{}, {}, {}], "max": [{}, {}, {}] }},
    {{ "bufferView": 1, "componentType": 5126, "count": 8, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5126, "count": 8, "type": "VEC4" }},
    {{ "bufferView": 3, "componentType": 5126, "count": 8, "type": "VEC2" }},
    {{ "bufferView": 4, "byteOffset": 0, "componentType": 5123, "count": 6, "type": "SCALAR" }},
    {{ "bufferView": 4, "byteOffset": 12, "componentType": 5123, "count": 6, "type": "SCALAR" }}
  ],
  "images": [ {{ "uri": "{グラデーション画像ファイル名}" }}, {{ "uri": "{雑音画像ファイル名}" }} ],
  "textures": [ {{ "source": 0 }}, {{ "source": 1 }} ],
  "materials": [
    {},
    {}
  ],
  "meshes": [ {{ "name": "ブロック圧縮の対照の板", "primitives": [
    {{ "attributes": {{ "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 }}, "indices": 4, "material": 0 }},
    {{ "attributes": {{ "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 }}, "indices": 5, "material": 1 }}
  ] }} ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}
"#,
        最小[0],
        最小[1],
        最小[2],
        最大[0],
        最大[1],
        最大[2],
        材質(0),
        材質(1)
    )
}
