//! 遠方環境の検収世界のglTF文書を組み立てる工程。受け取るのは板の一覧、返すのは1メッシュ8プリミティブの文書である。
//!
//! 板ごとにプリミティブと材質を分けるのは、判定が板ごとに違う金属度と粗さを要るためである。頂点属性のアクセサは
//! 8つのプリミティブが共有し、インデックスのアクセサだけを板ごとに分ける(材質境界の検収アセットと同じ作り)。

use super::indirect_probe_geometry::{
    インデックスのバイト長, テクスチャ座標のバイト長, 位置のバイト長, 位置の値域, 接線のバイト長, 板の枚数, 法線のバイト長,
};
use super::indirect_probe_plates::一覧;

pub(super) fn 文書() -> String {
    let (最小, 最大) = 位置の値域();
    let 法線の開始 = 位置のバイト長;
    let 接線の開始 = 法線の開始 + 法線のバイト長;
    let テクスチャ座標の開始 = 接線の開始 + 接線のバイト長;
    let インデックスの開始 = テクスチャ座標の開始 + テクスチャ座標のバイト長;
    let 全長 = インデックスの開始 + インデックスのバイト長;
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "uri": "indirect_probe.bin", "byteLength": {全長} }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": {位置のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {法線の開始}, "byteLength": {法線のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {接線の開始}, "byteLength": {接線のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {テクスチャ座標の開始}, "byteLength": {テクスチャ座標のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {インデックスの開始}, "byteLength": {インデックスのバイト長}, "target": 34963 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": {頂点数}, "type": "VEC3", "min": {最小文字列}, "max": {最大文字列} }},
    {{ "bufferView": 1, "componentType": 5126, "count": {頂点数}, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5126, "count": {頂点数}, "type": "VEC4" }},
    {{ "bufferView": 3, "componentType": 5126, "count": {頂点数}, "type": "VEC2" }},
    {インデックスアクセサ一覧}
  ],
  "materials": [
    {材質一覧}
  ],
  "meshes": [ {{ "name": "遠方環境の検収の板", "primitives": [
    {プリミティブ一覧}
  ] }} ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}
"#,
        頂点数 = 板の枚数 * 4,
        最小文字列 = 三成分を並べる(最小),
        最大文字列 = 三成分を並べる(最大),
        インデックスアクセサ一覧 = インデックスアクセサ一覧(),
        材質一覧 = 材質一覧(),
        プリミティブ一覧 = プリミティブ一覧(),
    )
}

fn インデックスアクセサ一覧() -> String {
    (0..板の枚数)
        .map(|板番号| {
            format!(
                r#"{{ "bufferView": 4, "byteOffset": {}, "componentType": 5123, "count": 6, "type": "SCALAR" }}"#,
                板番号 * 12
            )
        })
        .collect::<Vec<_>>()
        .join(",\n    ")
}

fn 材質一覧() -> String {
    一覧()
        .iter()
        .map(|板| {
            format!(
                r#"{{ "pbrMetallicRoughness": {{ "baseColorFactor": [{}, {}, {}, 1.0], "metallicFactor": {}, "roughnessFactor": {} }} }}"#,
                板.ベースカラー[0], 板.ベースカラー[1], 板.ベースカラー[2], 板.金属度, 板.粗さ
            )
        })
        .collect::<Vec<_>>()
        .join(",\n    ")
}

fn プリミティブ一覧() -> String {
    (0..板の枚数)
        .map(|板番号| {
            format!(
                r#"{{ "attributes": {{ "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 }}, "indices": {}, "material": {板番号} }}"#,
                板番号 + 4
            )
        })
        .collect::<Vec<_>>()
        .join(",\n    ")
}

fn 三成分を並べる(値: [f32; 3]) -> String {
    format!("[{}, {}, {}]", 値[0], 値[1], 値[2])
}
