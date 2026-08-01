//! ファイル1つを見るだけでは判定できない検査をまとめて走らせる工程。受け取るのは無し、
//! 返すのは違反一覧か、検査自体を実行できなかった理由である。
//!
//! ここに集めるのは、依存の白リスト・文書の節参照・CPU正本とslangの写しの一致・シェーダー定数の宣言箇所の4つである。
//! どれも複数のファイルを突き合わせて初めて判定でき、走査中の1ファイルからは答えが出ない。

use super::violation::違反;
use super::{dependency_whitelist, doc_section, shader_constant, shader_uniform_alias};

pub fn 集める() -> Result<Vec<違反>, String> {
    let mut 違反一覧 = Vec::new();
    違反一覧.extend(dependency_whitelist::全クレートを検査する().map_err(|誤り| format!("依存白リスト検査に失敗した: {誤り}"))?);
    違反一覧.extend(doc_section::全文書を検査する().map_err(|誤り| format!("節参照実在検査に失敗した: {誤り}"))?);
    違反一覧.extend(shader_constant::全定数を検査する().map_err(|誤り| format!("シェーダー定数の写し検査に失敗した: {誤り}"))?);
    違反一覧.extend(shader_uniform_alias::全シェーダーを検査する().map_err(|誤り| format!("シェーダー定数の宣言箇所の検査に失敗した: {誤り}"))?);
    Ok(違反一覧)
}
