//! ファイル1つを見るだけでは判定できない検査をまとめて走らせる工程。受け取るのは無し、
//! 返すのは違反一覧か、検査自体を実行できなかった理由である。
//!
//! ここに集めるのは、依存の白リスト・文書の節参照・正本と写しが同じ値を持つべき定数の一致・束縛番号の正本と写しの一致・報告の行の綴りの契約・ファイル名らしい綴りの重複・材質見本の宣言の写しの一致・シェーダー定数の宣言箇所・照明問い合わせのセットの宣言箇所・廃止した語である。
//! どれも複数のファイルを突き合わせて初めて判定でき、走査中の1ファイルからは答えが出ない。

use super::error::規約検査の破れ;
use super::violation::違反;
use super::{
    dependency_whitelist, depth_contract, doc_section, duplicate_file_literal, lighting_query_declaration, reload_without_device_wait,
    removed_object_uniform, removed_slot_material_set, removed_view_pass_lighting, sample_bodies_consistency, shader_binding, shader_constant,
    shader_uniform_alias, single_lighting_slot_write, wording_contract,
};

pub fn 集める() -> Result<Vec<違反>, 規約検査の破れ> {
    let mut 違反一覧 = Vec::new();
    違反一覧.extend(dependency_whitelist::全クレートを検査する()?);
    違反一覧.extend(depth_contract::全接点を検査する()?);
    違反一覧.extend(doc_section::全文書を検査する()?);
    違反一覧.extend(shader_constant::全定数を検査する()?);
    違反一覧.extend(shader_binding::全束縛番号を検査する()?);
    違反一覧.extend(duplicate_file_literal::全ファイルを検査する()?);
    違反一覧.extend(wording_contract::全綴りを検査する()?);
    違反一覧.extend(sample_bodies_consistency::全宣言を検査する()?);
    違反一覧.extend(shader_uniform_alias::全シェーダーを検査する()?);
    違反一覧.extend(lighting_query_declaration::全シェーダーを検査する()?);
    違反一覧.extend(removed_object_uniform::全ファイルを検査する()?);
    違反一覧.extend(removed_view_pass_lighting::全ファイルを検査する()?);
    違反一覧.extend(removed_slot_material_set::全ファイルを検査する()?);
    違反一覧.extend(reload_without_device_wait::検査する()?);
    違反一覧.extend(single_lighting_slot_write::検査する()?);
    Ok(違反一覧)
}
