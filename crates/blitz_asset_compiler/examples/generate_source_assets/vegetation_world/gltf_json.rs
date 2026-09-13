//! 植生の原型glTF文書。段の数だけメッシュを並べ、その並び順がそのまま段番号順(先頭=最詳細)になる。
//! マテリアルを持たないためベースカラーはエンジン側の既定テクスチャ(白)が使われ、番兵背景色との差が最大になる。
//! 直方体1つぶんのbufferViewとaccessorとmeshの宣言は`sections`が作り、ここは文書全体の骨組みだけを組む。

mod sections;

use super::geometry::直方体諸元;
use super::stage_amount::段の中身の量;

pub(super) fn 文書を作る(諸元一覧: &[直方体諸元], バッファファイル名: &str, 量: 段の中身の量) -> String {
    let バッファ長 = 量.バイト長() * 諸元一覧.len();
    let mut バッファビュー宣言一覧 = Vec::new();
    let mut アクセサ宣言一覧 = Vec::new();
    let mut メッシュ宣言一覧 = Vec::new();
    let mut ノード一覧 = Vec::new();
    let mut ノード番号一覧 = Vec::new();
    for (段番号, 諸元) in 諸元一覧.iter().enumerate() {
        let 区間 = sections::段の区間を作る(段番号, *諸元, 量);
        バッファビュー宣言一覧.push(区間.バッファビュー宣言);
        アクセサ宣言一覧.push(区間.アクセサ宣言);
        メッシュ宣言一覧.push(sections::メッシュ宣言を作る(段番号));
        ノード一覧.push(format!("{{ \"mesh\": {段番号} }}"));
        ノード番号一覧.push(段番号.to_string());
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
        バッファビュー宣言一覧.join(",\n"),
        アクセサ宣言一覧.join(",\n"),
        メッシュ宣言一覧.join(",\n"),
        ノード一覧.join(", "),
        ノード番号一覧.join(", ")
    )
}
