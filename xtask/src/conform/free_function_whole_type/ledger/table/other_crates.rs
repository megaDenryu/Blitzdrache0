//! `blitz_app`と`blitz_render`を除いたクレートの未是正の自由関数の一覧。件数が少ないため1つの表にまとめる。
//!
//! 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。削除できるのは、その工程が自分の触るものだけを
//! 名前の付いた引数で受け取る形へ直したときか、操作を親の型のメソッドへ移したときだけである。

use super::super::{区画の一覧, 未是正の自由関数};

const 項目一覧: [未是正の自由関数; 15] = [
    未是正の自由関数::生成する(
        "blitz_asset_compiler/src/loader/contract/archetype_fixture.rs",
        "原型の文書jsonを作る",
        "原型の文書の指定",
    ),
    未是正の自由関数::生成する("blitz_asset_compiler/src/loader/contract/fixture_json.rs", "文書jsonを作る", "文書の指定"),
    未是正の自由関数::生成する(
        "blitz_asset_compiler/src/runtime_compilation/source_location.rs",
        "ソースパスを参照する",
        "外部ソースルート",
    ),
    未是正の自由関数::生成する(
        "blitz_asset_compiler/src/runtime_compilation/world/assembled_scatter_declaration.rs",
        "指定を1件作る",
        "部品で組む散布の宣言",
    ),
    未是正の自由関数::生成する(
        "blitz_collision/src/height_field/segment_query_fixture.rs",
        "当たりを求める",
        "検査用の四隅の供給元",
    ),
    未是正の自由関数::生成する("blitz_engine/src/streaming/chunk_ledger/eviction.rs", "目標使用量", "チャンク台帳"),
    未是正の自由関数::生成する("blitz_engine/src/streaming/chunk_ledger/eviction.rs", "退避を計画する", "チャンク台帳"),
    未是正の自由関数::生成する(
        "blitz_engine/src/streaming/chunk_ledger/eviction_victim.rs",
        "次の犠牲を選ぶ",
        "チャンク台帳",
    ),
    未是正の自由関数::生成する(
        "blitz_engine/src/streaming/chunk_ledger/usage.rs",
        "記録から使用量を求める",
        "チャンク記録",
    ),
    未是正の自由関数::生成する("blitz_sim/src/constraint_graph/grid.rs", "規則格子の拘束グラフを作る", "規則格子の仕様"),
    未是正の自由関数::生成する(
        "blitz_sim/src/constraint_graph/irregular.rs",
        "不規則な拘束グラフを作る",
        "不規則グラフの仕様",
    ),
    未是正の自由関数::生成する(
        "editor_server/src/building_grid_store/error.rs",
        "状態と種別を決める",
        "建物の格子の保存エラー",
    ),
    未是正の自由関数::生成する(
        "editor_server/src/resource/building_outline_catalog/validation.rs",
        "入口方向を利用できるか",
        "建物外形定義",
    ),
    未是正の自由関数::生成する(
        "editor_server/src/resource/building_outline_catalog/validation.rs",
        "定義を検証する",
        "建物外形定義",
    ),
    未是正の自由関数::生成する("editor_server/src/storage/response.rs", "状態と種別を決める", "保存要求エラー"),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する("crates", &項目一覧)
}
