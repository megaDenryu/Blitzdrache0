//! 材質見本の立体のglTF文書を組み立てる工程。受け取るのは連結した立体とバイナリのファイル名、返すのは1メッシュ9プリミティブの文書である。
//!
//! 立体ごとにプリミティブと材質を分けるのは、球8つと台座が別々の金属度と粗さを持つためである。頂点属性のアクセサは
//! 9つのプリミティブが共有し、インデックスのアクセサだけを立体ごとに分ける(遠方環境の検収アセットと同じ作り)。

use super::geometry::連結した立体;
use super::sample_bodies::{台座のベースカラー, 台座の粗さ, 台座の金属度, 球の一覧};

/// インデックスのアクセサが始まる番号。0が位置、1が法線である。
const インデックスアクセサの起点: usize = 2;

pub(super) fn 文書(連結: &連結した立体, バイナリファイル名: &str) -> String {
    let (最小, 最大) = 連結.位置の値域();
    let 法線の開始 = 連結.位置のバイト長();
    let インデックスの開始 = 法線の開始 + 連結.法線のバイト長();
    let 全長 = インデックスの開始 + 連結.インデックスのバイト長();
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "uri": "{バイナリファイル名}", "byteLength": {全長} }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": {位置のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {法線の開始}, "byteLength": {法線のバイト長}, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": {インデックスの開始}, "byteLength": {インデックスのバイト長}, "target": 34963 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": {頂点数}, "type": "VEC3", "min": {最小文字列}, "max": {最大文字列} }},
    {{ "bufferView": 1, "componentType": 5126, "count": {頂点数}, "type": "VEC3" }},
    {インデックスアクセサ一覧}
  ],
  "materials": [
    {材質一覧}
  ],
  "meshes": [ {{ "name": "材質見本の立体", "primitives": [
    {プリミティブ一覧}
  ] }} ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}
"#,
        位置のバイト長 = 連結.位置のバイト長(),
        法線のバイト長 = 連結.法線のバイト長(),
        インデックスのバイト長 = 連結.インデックスのバイト長(),
        頂点数 = 連結.頂点数(),
        最小文字列 = 三成分を並べる(最小),
        最大文字列 = 三成分を並べる(最大),
        インデックスアクセサ一覧 = インデックスアクセサ一覧(連結),
        材質一覧 = 材質一覧(),
        プリミティブ一覧 = プリミティブ一覧(連結),
    )
}

fn インデックスアクセサ一覧(連結: &連結した立体) -> String {
    連結
        .区間一覧
        .iter()
        .map(|区間| {
            format!(
                r#"{{ "bufferView": 2, "byteOffset": {}, "componentType": 5123, "count": {}, "type": "SCALAR" }}"#,
                区間.添字ずらし量 * 2,
                区間.添字数
            )
        })
        .collect::<Vec<_>>()
        .join(",\n    ")
}

fn 材質一覧() -> String {
    let mut 行一覧: Vec<String> = 球の一覧().iter().map(|球| 材質を書く(球.ベースカラー, 球.金属度, 球.粗さ)).collect();
    行一覧.push(材質を書く(台座のベースカラー, 台座の金属度, 台座の粗さ));
    行一覧.join(",\n    ")
}

fn 材質を書く(ベースカラー: [f32; 3], 金属度: f32, 粗さ: f32) -> String {
    format!(
        r#"{{ "pbrMetallicRoughness": {{ "baseColorFactor": [{}, {}, {}, 1.0], "metallicFactor": {金属度}, "roughnessFactor": {粗さ} }} }}"#,
        ベースカラー[0], ベースカラー[1], ベースカラー[2]
    )
}

fn プリミティブ一覧(連結: &連結した立体) -> String {
    (0..連結.区間一覧.len())
        .map(|立体番号| {
            format!(
                r#"{{ "attributes": {{ "POSITION": 0, "NORMAL": 1 }}, "indices": {}, "material": {立体番号} }}"#,
                立体番号 + インデックスアクセサの起点
            )
        })
        .collect::<Vec<_>>()
        .join(",\n    ")
}

fn 三成分を並べる(値: [f32; 3]) -> String {
    format!("[{}, {}, {}]", 値[0], 値[1], 値[2])
}
