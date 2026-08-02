//! ファイル1つを見るだけでは判定できない検査をまとめて走らせる工程。受け取るのは無し、
//! 返すのは違反一覧か、検査自体を実行できなかった理由である。
//!
//! ここに集めるのは、依存の白リスト・文書の節参照・CPU正本とslangの写しの一致・シェーダー定数の宣言箇所・照明問い合わせのセットの宣言箇所・廃止した語の3種である。
//! どれも複数のファイルを突き合わせて初めて判定でき、走査中の1ファイルからは答えが出ない。

use super::violation::違反;
use super::{
    dependency_whitelist, doc_section, lighting_query_declaration, reload_without_device_wait, removed_object_uniform, removed_slot_material_set,
    removed_view_pass_lighting, shader_constant, shader_uniform_alias, single_lighting_slot_write,
};

pub fn 集める() -> Result<Vec<違反>, String> {
    let mut 違反一覧 = Vec::new();
    違反一覧.extend(dependency_whitelist::全クレートを検査する().map_err(|誤り| format!("依存白リスト検査に失敗した: {誤り}"))?);
    違反一覧.extend(doc_section::全文書を検査する().map_err(|誤り| format!("節参照実在検査に失敗した: {誤り}"))?);
    違反一覧.extend(shader_constant::全定数を検査する().map_err(|誤り| format!("シェーダー定数の写し検査に失敗した: {誤り}"))?);
    違反一覧.extend(shader_uniform_alias::全シェーダーを検査する().map_err(|誤り| format!("シェーダー定数の宣言箇所の検査に失敗した: {誤り}"))?);
    違反一覧
        .extend(lighting_query_declaration::全シェーダーを検査する().map_err(|誤り| format!("照明問い合わせの宣言箇所の検査に失敗した: {誤り}"))?);
    違反一覧.extend(removed_object_uniform::全ファイルを検査する().map_err(|誤り| format!("廃止語の検査に失敗した: {誤り}"))?);
    違反一覧
        .extend(removed_view_pass_lighting::全ファイルを検査する().map_err(|誤り| format!("旧ビュー定数の照明の廃止語の検査に失敗した: {誤り}"))?);
    違反一覧.extend(removed_slot_material_set::全ファイルを検査する().map_err(|誤り| format!("旧材質セットの廃止語の検査に失敗した: {誤り}"))?);
    違反一覧.extend(reload_without_device_wait::検査する().map_err(|誤り| format!("シーンの差し替えの待ちの検査に失敗した: {誤り}"))?);
    違反一覧.extend(single_lighting_slot_write::検査する().map_err(|誤り| format!("照明スロットの書き込み地点の検査に失敗した: {誤り}"))?);
    Ok(違反一覧)
}
