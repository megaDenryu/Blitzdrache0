//! 材質境界の検収シーンのglTF文書。同じ形を2材質2プリミティブで塗る文書と、1材質1プリミティブで塗る対照の文書を作る。
//! bufferViewsの位置と長さは`multi_material_geometry`の書き出し順(位置96B→法線96B→接線128B→UV64B→インデックス24B、計408B)へ手で一致させている。
//! 頂点属性のアクセサを2つのプリミティブが共有するのは、2つの文書の違いを材質の境界だけに限るためである。
//! 検収は左右の板の画素を比べるため、2つの材質はベースカラー係数だけを大きく変えて、どちらの画素かを色で判別できるようにしてある。

const 左の材質: &str = r#"{ "pbrMetallicRoughness": { "baseColorFactor": [0.9, 0.12, 0.1, 1.0], "metallicFactor": 0.0, "roughnessFactor": 0.8 } }"#;
const 右の材質: &str = r#"{ "pbrMetallicRoughness": { "baseColorFactor": [0.1, 0.2, 0.9, 1.0], "metallicFactor": 0.0, "roughnessFactor": 0.8 } }"#;

/// 差し替え後の材質。左右とも元の色と3成分すべてで大きく離れた色を選び、差し替えの前後の画素を取り違えないようにしてある。
const 代替の左の材質: &str =
    r#"{ "pbrMetallicRoughness": { "baseColorFactor": [0.1, 0.75, 0.2, 1.0], "metallicFactor": 0.0, "roughnessFactor": 0.8 } }"#;
const 代替の右の材質: &str =
    r#"{ "pbrMetallicRoughness": { "baseColorFactor": [0.85, 0.7, 0.1, 1.0], "metallicFactor": 0.0, "roughnessFactor": 0.8 } }"#;

/// 2材質2プリミティブの文書。左の板が材質0、右の板が材質1である。
pub(super) fn 二材質の文書() -> String {
    二材質を並べる(左の材質, 右の材質)
}

/// 材質の係数だけを差し替えた2材質2プリミティブの文書。形もプリミティブの数も`二材質の文書`と同じであり、
/// 実行時形式を上書きしても変わるのは板の色だけになる。資源表世代の切替が画素に出ることの検収が読む。
pub(super) fn 代替の二材質の文書() -> String {
    二材質を並べる(代替の左の材質, 代替の右の材質)
}

fn 二材質を並べる(左: &str, 右: &str) -> String {
    let インデックスアクセサ一覧 = concat!(
        r#"{ "bufferView": 4, "byteOffset": 0, "componentType": 5123, "count": 6, "type": "SCALAR" },"#,
        r#"{ "bufferView": 4, "byteOffset": 12, "componentType": 5123, "count": 6, "type": "SCALAR" }"#
    );
    let プリミティブ一覧 = concat!(
        r#"{ "attributes": { "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 }, "indices": 4, "material": 0 },"#,
        r#"{ "attributes": { "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 }, "indices": 5, "material": 1 }"#
    );
    文書を作る(&format!("{左},\n    {右}"), インデックスアクセサ一覧, プリミティブ一覧)
}

/// 同じ形を1材質1プリミティブで塗る対照の文書。発行数と可視ID数の比較相手になる。
pub(super) fn 単一材質の文書() -> String {
    let インデックスアクセサ一覧 = r#"{ "bufferView": 4, "byteOffset": 0, "componentType": 5123, "count": 12, "type": "SCALAR" }"#;
    let プリミティブ一覧 = r#"{ "attributes": { "POSITION": 0, "NORMAL": 1, "TANGENT": 2, "TEXCOORD_0": 3 }, "indices": 4, "material": 0 }"#;
    文書を作る(左の材質, インデックスアクセサ一覧, プリミティブ一覧)
}

fn 文書を作る(材質一覧: &str, インデックスアクセサ一覧: &str, プリミティブ一覧: &str) -> String {
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [ {{ "uri": "multi_material.bin", "byteLength": 408 }} ],
  "bufferViews": [
    {{ "buffer": 0, "byteOffset": 0, "byteLength": 96, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 96, "byteLength": 96, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 192, "byteLength": 128, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 320, "byteLength": 64, "target": 34962 }},
    {{ "buffer": 0, "byteOffset": 384, "byteLength": 24, "target": 34963 }}
  ],
  "accessors": [
    {{ "bufferView": 0, "componentType": 5126, "count": 8, "type": "VEC3", "min": [-1.2, -0.6, 0.0], "max": [1.2, 0.6, 0.0] }},
    {{ "bufferView": 1, "componentType": 5126, "count": 8, "type": "VEC3" }},
    {{ "bufferView": 2, "componentType": 5126, "count": 8, "type": "VEC4" }},
    {{ "bufferView": 3, "componentType": 5126, "count": 8, "type": "VEC2" }},
    {インデックスアクセサ一覧}
  ],
  "materials": [
    {材質一覧}
  ],
  "meshes": [ {{ "name": "材質境界の板", "primitives": [
    {プリミティブ一覧}
  ] }} ],
  "nodes": [ {{ "mesh": 0 }} ],
  "scenes": [ {{ "nodes": [0] }} ],
  "scene": 0
}}
"#
    )
}
