//! 材質の絵の検収系コマンドの割り当て。command_catalogの`material_check`分類と同じ範囲(材質のデータが
//! 実機の画素まで届いていることを確かめる4件)を担当する。

use std::process::ExitCode;

use crate::material_check::{material_reload_draw, multi_material_draw, prop_multi_material_draw, surface_layer_draw};

pub(super) fn 材質検収コマンドを割り当てる(名前: &str) -> Option<ExitCode> {
    match 名前 {
        "prop-multi-material-draw" => Some(prop_multi_material_draw::多材質小物の描画を確認する()),
        "multi-material-draw" => Some(multi_material_draw::多材質シーンの描画を確認する()),
        "surface-layer-draw" => Some(surface_layer_draw::地表の層の重ね合わせの描画を確認する()),
        "material-reload-draw" => Some(material_reload_draw::材質差し替えの描画を確認する()),
        _ => None,
    }
}
