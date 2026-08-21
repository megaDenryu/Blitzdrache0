//! 材質の絵の検収の入口をまとめた分類。担当するのは、材質のデータが実機の画素まで届いていることを確かめる
//! 4つの入口を1つの木の下へ置くことだけであり、判定の中身は各入口が持つ。
//!
//! 4つを1つの分類にするのは、どれも「材質が絵にどう出るか」を見る入口であり、材質の形式や資源表を変えたときに
//! 一緒に走らせる範囲だからである。分類はコマンド一覧(`command_ui::command_catalog::material_check`)と
//! 割り当て(`dispatch::material_check`)の分け方と同じである。

pub(crate) mod material_reload_draw;
pub(crate) mod multi_material_draw;
pub(crate) mod prop_multi_material_draw;
pub(crate) mod surface_layer_draw;
