//! 群の原型の試験材料。担当するのは段の列の文書JSONと、その文書が指すバイナリの両方を組み立てることである。
//! 差し替えられるのはメッシュ列とノード列の2箇所であり、負の対照はそこだけを崩して作る。
//!
//! JSONとバイナリを1つのファイルが持つのは、バッファビューの位置と長さがバイナリの並び(接地した位置72B・持ち上げた位置72B・
//! 法線72B・UV48B・インデックス6個12B・インデックス3個6B)へ手で一致させたものであり、片方だけを直すと材料が黙って壊れるためである。
//!
//! 位置のアクセサを接地した組と持ち上げた組の2つ持つのは、原点が接地面にあるかどうかの負の対照を、宣言だけでなく頂点の実体ごと入れ替えて作るためである。

use super::fixture_json::不透明の茶色;

pub(super) struct 原型の文書の指定<'材料> {
    pub(super) メッシュ列: &'材料 str,
    pub(super) ノード列: &'材料 str,
}

/// 段を並べてメッシュ列にする。段の並び順そのものが検査の対象であるため、並べ方は試験の側が1件ずつ書く。
pub(super) fn 段を並べる(段一覧: &[&str]) -> String {
    段一覧.join(", ")
}

/// 接地した位置を使い、三角形2つを持つ段。
pub(super) const 接地した三角形2つの段: &str = r#"{ "name": "三角形2つの段", "primitives": [ { "attributes": { "POSITION": 0, "NORMAL": 2, "TEXCOORD_0": 3 }, "indices": 4, "material": 0 } ] }"#;

/// 接地した位置を使い、三角形1つを持つ段。
pub(super) const 接地した三角形1つの段: &str = r#"{ "name": "三角形1つの段", "primitives": [ { "attributes": { "POSITION": 0, "NORMAL": 2, "TEXCOORD_0": 3 }, "indices": 5, "material": 0 } ] }"#;

/// 位置ごと真上へ持ち上げた、三角形1つの段。底面が接地面から離れている。
pub(super) const 浮いた三角形1つの段: &str =
    r#"{ "name": "浮いた段", "primitives": [ { "attributes": { "POSITION": 1, "NORMAL": 2, "TEXCOORD_0": 3 }, "indices": 5, "material": 0 } ] }"#;

/// UV座標(TEXCOORD_0)を宣言しない、三角形1つの段。UV座標を持つ段と並べると、段の間で描画条件が揃わなくなる。
pub(super) const UV座標を持たない三角形1つの段: &str =
    r#"{ "name": "UVなしの段", "primitives": [ { "attributes": { "POSITION": 0, "NORMAL": 2 }, "indices": 5, "material": 0 } ] }"#;

pub(super) const 変換を持たない1ノード: &str = r#"{ "mesh": 0 }"#;

pub(super) const 変換を持たない2ノード: &str = r#"{ "mesh": 0 }, { "mesh": 1 }"#;

/// 底面が高さ0にあり、水平の中心が原点にある四角錐の6頂点。x・y・zの順に並べた成分である。
const 接地した位置の成分: [f32; 18] = [
    -1.0, 0.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 1.0, 0.0,
];

pub(super) fn 原型の文書jsonを作る(指定: &原型の文書の指定<'_>) -> String {
    let 原型の文書の指定 {
        メッシュ列, ノード列
    } = 指定;
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "byteLength": 282 }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": 72 }},
    {{ "buffer": 0, "byteOffset": 72, "byteLength": 72 }},
    {{ "buffer": 0, "byteOffset": 144, "byteLength": 72 }},
    {{ "buffer": 0, "byteOffset": 216, "byteLength": 48 }},
    {{ "buffer": 0, "byteOffset": 264, "byteLength": 12 }},
    {{ "buffer": 0, "byteOffset": 276, "byteLength": 6 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": 6, "type": "VEC3", "min": [-1.0, 0.0, -1.0], "max": [1.0, 2.0, 1.0] }},
    {{ "bufferView": 1, "componentType": 5126, "count": 6, "type": "VEC3", "min": [-1.0, 1.0, -1.0], "max": [1.0, 3.0, 1.0] }},
    {{ "bufferView": 2, "componentType": 5126, "count": 6, "type": "VEC3" }},
    {{ "bufferView": 3, "componentType": 5126, "count": 6, "type": "VEC2" }},
    {{ "bufferView": 4, "componentType": 5123, "count": 6, "type": "SCALAR" }},
    {{ "bufferView": 5, "componentType": 5123, "count": 3, "type": "SCALAR" }}
  ],
  "materials": [ {不透明の茶色} ],
  "meshes": [ {メッシュ列} ],
  "nodes": [ {ノード列} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}"#
    )
}

/// 接地した位置6頂点・真上へ1だけ持ち上げた位置6頂点・法線6頂点・UV6頂点・インデックス6個・インデックス3個を、この順に並べたバイナリ。
pub(super) fn 原型の段の列のバイナリ() -> Vec<u8> {
    let mut バイト列 = Vec::new();
    for 持ち上げ in [0.0f32, 1.0] {
        for (添字, 値) in 接地した位置の成分.iter().enumerate() {
            let 成分 = if 添字 % 3 == 1 { 値 + 持ち上げ } else { *値 };
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
    }
    for 値 in [0.0f32, 1.0, 0.0].repeat(6) {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    for 値 in [0.0f32, 0.0].repeat(6) {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    for 値 in [0u16, 1, 2, 0, 2, 3, 0, 1, 2] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    バイト列
}
