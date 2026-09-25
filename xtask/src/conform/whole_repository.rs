//! ファイル1つを見るだけでは判定できない検査をまとめて走らせる工程。受け取るのは無し、
//! 返すのは違反と警告の報告か、検査自体を実行できなかった理由である。
//!
//! ここに集めるのは、依存の白リスト・依存の機能一覧の台帳・文書の節参照・正本と写しが同じ値を持つべき定数の一致・束縛番号の正本と写しの一致・シェーダーの原文に在るべき綴りと在ってはならない綴り・報告の行の綴りの契約・ファイル名らしい綴りの重複・材質見本の宣言の写しの一致・シェーダー定数の宣言箇所・照明問い合わせのセットの宣言箇所・廃止した語・型ごとの分量の台帳・検証の出力が載る木を指す綴りの置き場・設計オントロジーの法則(警告を1つ含む)・設計関係の抽出器が受理する正規形のRustの構文である。
//! どれも複数のファイルを突き合わせて初めて判定でき、走査中の1ファイルからは答えが出ない。

use super::error::規約検査の破れ;
use super::report::検査の報告;
use super::{
    dependency_whitelist, depth_contract, design_ontology, doc_section, duplicate_file_literal, extractable_normal_form, free_function_whole_type, lighting_query_declaration, reload_without_device_wait, removed_object_uniform,
    removed_slot_material_set, removed_view_pass_lighting, sample_bodies_consistency, shader_binding, shader_constant, shader_form, shader_uniform_alias, single_lighting_slot_write, type_metrics_ledger, verify_output_place, wording_contract,
    workspace_dependency_features,
};

pub fn 複数ファイルを横断する検査の違反一覧を集める() -> Result<検査の報告, 規約検査の破れ> {
    let mut 違反一覧 = Vec::new();
    違反一覧.extend(dependency_whitelist::全クレートを検査する()?);
    違反一覧.extend(workspace_dependency_features::全依存を検査する()?);
    違反一覧.extend(depth_contract::全接点を検査する()?);
    違反一覧.extend(doc_section::全文書を検査する()?);
    違反一覧.extend(shader_constant::全定数を検査する()?);
    違反一覧.extend(shader_binding::全束縛番号を検査する()?);
    違反一覧.extend(shader_form::全シェーダーの原文の形を検査する()?);
    違反一覧.extend(duplicate_file_literal::全ファイルを検査する()?);
    違反一覧.extend(wording_contract::全文言を検査する()?);
    違反一覧.extend(sample_bodies_consistency::全宣言を検査する()?);
    違反一覧.extend(shader_uniform_alias::全シェーダーを検査する()?);
    違反一覧.extend(lighting_query_declaration::全シェーダーを検査する()?);
    違反一覧.extend(removed_object_uniform::全ファイルを検査する()?);
    違反一覧.extend(removed_view_pass_lighting::全ファイルを検査する()?);
    違反一覧.extend(removed_slot_material_set::全ファイルを検査する()?);
    違反一覧.extend(reload_without_device_wait::シーン差し替えのgpu全作業完了待ちを検査する()?);
    違反一覧.extend(single_lighting_slot_write::照明問い合わせスロットへの書き込み元を検査する()?);
    違反一覧.extend(type_metrics_ledger::全型の分量を台帳と照合する()?);
    違反一覧.extend(free_function_whole_type::全ファイルの自由関数を検査する()?);
    違反一覧.extend(verify_output_place::全ファイルを検査する()?);
    違反一覧.extend(extractable_normal_form::全ソースを検査する()?);
    Ok(検査の報告::生成する(違反一覧, Vec::new()).合わせる(design_ontology::全ファイルを検査する()?))
}
