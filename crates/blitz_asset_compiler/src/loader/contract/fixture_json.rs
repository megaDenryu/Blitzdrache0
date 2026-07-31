//! 試験材料のglTF文書JSONの雛形。担当するのはノード列・プリミティブ列・マテリアル列・追加メッシュ列の4箇所だけを差し替えられる文書の組み立てである。
//! バッファビューの位置と長さは`glb_fixture`の`三角形のバイナリ`の並び(位置36B・法線36B・UV24B・インデックス6B)へ手で一致させている。
//! 複数の材質の材料が同じ頂点・インデックスのアクセサを2つのプリミティブで共有するのは、材質の検査に形の違いが要らないためである。

pub(super) struct 文書の指定<'a> {
    pub(super) ノード列: &'a str,
    pub(super) プリミティブ列: &'a str,
    pub(super) マテリアル列: &'a str,
    /// 2つ目以降のメッシュ。先頭に読点を付けた形で渡す。空文字列なら1メッシュの文書になる。
    pub(super) 追加メッシュ列: &'a str,
}

/// どの検査項目にも触れない文書の指定。負の対照はこれを1箇所だけ崩して作る。
pub(super) fn 合格の指定() -> 文書の指定<'static> {
    文書の指定 {
        ノード列: r#"{ "mesh": 0 }"#,
        プリミティブ列: 三角形のプリミティブ,
        マテリアル列: 不透明の茶色,
        追加メッシュ列: "",
    }
}

pub(super) const 三角形のプリミティブ: &str = r#"{ "attributes": { "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 }, "indices": 3, "material": 0 }"#;

pub(super) const 不透明の茶色: &str =
    r#"{ "doubleSided": true, "pbrMetallicRoughness": { "baseColorFactor": [0.8, 0.3, 0.2, 1.0], "metallicFactor": 0.5, "roughnessFactor": 0.4 } }"#;

/// 2材質の材料。2つ目は色だけが違い、どの検査項目にも触れない。
pub(super) fn 二材質の指定() -> 文書の指定<'static> {
    文書の指定 {
        ノード列: r#"{ "mesh": 0 }"#,
        プリミティブ列: 二材質のプリミティブ列,
        マテリアル列: 二材質の不透明列,
        追加メッシュ列: "",
    }
}

const 二材質のプリミティブ列: &str = concat!(
    r#"{ "attributes": { "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 }, "indices": 3, "material": 0 }, "#,
    r#"{ "attributes": { "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 }, "indices": 3, "material": 1 }"#
);

const 二材質の不透明列: &str = concat!(
    r#"{ "doubleSided": true, "pbrMetallicRoughness": { "baseColorFactor": [0.8, 0.3, 0.2, 1.0], "metallicFactor": 0.5, "roughnessFactor": 0.4 } }, "#,
    r#"{ "doubleSided": true, "pbrMetallicRoughness": { "baseColorFactor": [0.1, 0.2, 0.9, 1.0], "metallicFactor": 1.0, "roughnessFactor": 0.3 } }"#
);

pub(super) fn 文書jsonを作る(指定: &文書の指定<'_>) -> String {
    let 文書の指定 {
        ノード列,
        プリミティブ列,
        マテリアル列,
        追加メッシュ列,
    } = 指定;
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "byteLength": 102 }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": 36 }},
    {{ "buffer": 0, "byteOffset": 36, "byteLength": 36 }},
    {{ "buffer": 0, "byteOffset": 72, "byteLength": 24 }},
    {{ "buffer": 0, "byteOffset": 96, "byteLength": 6 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3", "min": [0.0, 0.0, 0.0], "max": [1.0, 1.0, 0.0] }},
    {{ "bufferView": 1, "componentType": 5126, "count": 3, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5126, "count": 3, "type": "VEC2" }},
    {{ "bufferView": 3, "componentType": 5123, "count": 3, "type": "SCALAR" }}
  ],
  "materials": [ {マテリアル列} ],
  "meshes": [ {{ "name": "検査用三角形", "primitives": [ {プリミティブ列} ] }}{追加メッシュ列} ],
  "nodes": [ {ノード列} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}"#
    )
}
