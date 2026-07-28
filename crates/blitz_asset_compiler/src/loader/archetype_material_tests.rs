//! 原型の段間の描画条件の検査を、揃った2段・マテリアルが食い違う2段・UV座標の有無が食い違う2段で確かめる。
//! バッファ実体を解決せずに文書だけで判定できるため、検証用のglTFはJSONを組み立ててその場で解析する。

use super::archetype_material::全段が同じ描画条件かを検査する;
use crate::error::アセットコンパイルエラー;

/// 先頭段はマテリアル0とUV座標を持つ。後続段の宣言だけを引数で変える。
fn 二段の文書を作る(後続段のマテリアル番号: usize, 後続段がuv座標を持つか: bool) -> gltf::Document {
    let 後続段のuv = if 後続段がuv座標を持つか { r#", "TEXCOORD_0": 1"# } else { "" };
    let json = format!(
        r#"{{
          "asset": {{ "version": "2.0" }},
          "buffers": [{{ "byteLength": 20, "uri": "unused.bin" }}],
          "bufferViews": [{{ "buffer": 0, "byteOffset": 0, "byteLength": 20 }}],
          "accessors": [
            {{ "bufferView": 0, "componentType": 5126, "count": 1, "type": "VEC3", "min": [0, 0, 0], "max": [1, 1, 1] }},
            {{ "bufferView": 0, "componentType": 5126, "count": 1, "type": "VEC2" }}
          ],
          "materials": [{{}}, {{}}],
          "meshes": [
            {{ "primitives": [{{ "attributes": {{ "POSITION": 0, "TEXCOORD_0": 1 }}, "material": 0 }}] }},
            {{ "primitives": [{{ "attributes": {{ "POSITION": 0{後続段のuv} }}, "material": {後続段のマテリアル番号} }}] }}
          ]
        }}"#
    );
    match gltf::Gltf::from_slice(json.as_bytes()) {
        Ok(gltf::Gltf { document, .. }) => document,
        Err(誤り) => panic!("検証用glTFを解析できなかった: {誤り}"),
    }
}

#[test]
fn 全段が同じマテリアルを参照する原型を受理する() {
    assert!(全段が同じ描画条件かを検査する(&二段の文書を作る(0, true)).is_ok());
}

/// 後続段のマテリアルを無言で捨てて先頭段のものを全段へ適用しないため、ここで止める。
#[test]
fn 段ごとに違うマテリアルを持つ原型を拒否する() {
    let 結果 = 全段が同じ描画条件かを検査する(&二段の文書を作る(1, true));
    assert!(
        matches!(結果, Err(アセットコンパイルエラー::原型の段間マテリアル不一致 { 段番号: 1 })),
        "{結果:?}"
    );
}

/// マテリアルがテクスチャを読むとき、UV座標を持たない段だけが原点のテクセルで塗られることになる。
#[test]
fn uv座標の有無が段で食い違う原型を拒否する() {
    let 結果 = 全段が同じ描画条件かを検査する(&二段の文書を作る(0, false));
    assert!(
        matches!(結果, Err(アセットコンパイルエラー::原型の段間UV有無不一致 { 段番号: 1 })),
        "{結果:?}"
    );
}
