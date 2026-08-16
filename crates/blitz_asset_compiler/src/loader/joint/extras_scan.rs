//! ノードのextrasから接合点の宣言の並びを取り出す工程。受け取るのはglTFのノード、返すのは宣言1件ずつのJSONの列である。
//!
//! 宣言の不在を破れにしないのは、接合点を持たない部品でないアセット(単体で置く小物)が正当に成立し、
//! それを違反とするかは契約の側が決めることだからである。ここは取り出すだけで、何が違反かを持たない。
//!
//! extrasのキーの綴りをここが1箇所で持つ。Blender側の生成スクリプトが綴る名前と同じである。

use serde_json::Value;

use super::error::接合点読み取りエラー;

const 接合点のキー: &str = "接合点";

/// extrasが無い、または接合点のキーが無いときは空の並びを返す。
pub(super) fn 接合点の宣言の並びを取り出す(ノード: &gltf::Node<'_>) -> Result<Vec<Value>, 接合点読み取りエラー> {
    let Some(宣言の綴り) = ノード.extras().as_ref() else {
        return Ok(Vec::new());
    };
    let 解いた値: Value = serde_json::from_str(宣言の綴り.get())
        .map_err(|誤り| 接合点読み取りエラー::宣言のJSONを解けない { 誤り: 誤り.to_string() })?;
    let Some(宣言) = 解いた値.get(接合点のキー) else {
        return Ok(Vec::new());
    };
    宣言.as_array().cloned().ok_or(接合点読み取りエラー::接合点の宣言が配列でない)
}
