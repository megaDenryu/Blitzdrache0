//! 植生の原型glTF文書。段の数だけメッシュを並べ、その並び順がそのまま段番号順(先頭=最詳細)になる。
//! マテリアルを持たないためベースカラーはエンジン側の既定テクスチャ(白)が使われ、番兵背景色との差が最大になる。
//! 直方体1つぶんのbufferViewとaccessorとmeshの宣言は`sections`が作り、ここは文書全体の骨組みだけを組む。

mod sections;

use super::geometry::{直方体のバイト長, 直方体諸元};

pub(super) fn 文書を作る(諸元一覧: &[直方体諸元], バッファファイル名: &str) -> String {
    let バッファ長 = 直方体のバイト長 * 諸元一覧.len();
    let mut bufferview一覧 = Vec::new();
    let mut accessor一覧 = Vec::new();
    let mut mesh一覧 = Vec::new();
    let mut node一覧 = Vec::new();
    let mut node番号一覧 = Vec::new();
    for (段番号, 諸元) in 諸元一覧.iter().enumerate() {
        let 区間 = sections::段の区間を作る(段番号, *諸元);
        bufferview一覧.push(区間.bufferview宣言);
        accessor一覧.push(区間.accessor宣言);
        mesh一覧.push(sections::mesh宣言を作る(段番号));
        node一覧.push(format!("{{ \"mesh\": {段番号} }}"));
        node番号一覧.push(段番号.to_string());
    }
    format!(
        r#"{{
  "asset": {{ "version": "2.0" }},
  "buffers": [
    {{ "uri": "{バッファファイル名}", "byteLength": {バッファ長} }}
  ],
  "bufferViews": [
{}
  ],
  "accessors": [
{}
  ],
  "meshes": [
{}
  ],
  "nodes": [ {} ],
  "scenes": [ {{ "nodes": [{}] }} ],
  "scene": 0
}}
"#,
        bufferview一覧.join(",\n"),
        accessor一覧.join(",\n"),
        mesh一覧.join(",\n"),
        node一覧.join(", "),
        node番号一覧.join(", ")
    )
}
