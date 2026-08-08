//! シーン名と起動指定から決まる空の方針・間接照明方針・時間再構成方式を検査する。検証世界が空を持たないことと、
//! 2つの上書きが別の効き方をすることと、天空の遠方環境を選ぶ世界が地形世界と検収世界に限られることと、
//! 履歴混合を選ぶ世界が地形世界だけであることを固定する。

use blitz_engine::sky::{世界の空方針, 世界の間接照明方針};
use blitz_engine::temporal_reconstruction::時間再構成方式;

use super::scene_policy::{
    世界の時間再構成方式を決める, 世界の空方針を決める, 世界の間接照明方針を決める, 空を描くか
};
use crate::cli::temporal_reconstruction_settings::時間再構成方式の起動上書き;
use crate::cli::空の起動指定;

fn 空ありか(方針: 世界の空方針) -> bool {
    matches!(方針, 世界の空方針::空あり { .. })
}

#[test]
fn 地形世界だけが空を持ち検証世界は持たない() {
    assert!(空ありか(世界の空方針を決める("terrain_origin", 空の起動指定::方針に従う)));
    for シーン名 in ["quad", "helmet", "shadow_scene", "fox", "vegetation_cull"] {
        assert!(!空ありか(世界の空方針を決める(シーン名, 空の起動指定::方針に従う)), "{シーン名}");
    }
}

#[test]
fn 空ありとして扱う指定は方針そのものを上書きする() {
    let 方針 = 世界の空方針を決める("quad", 空の起動指定::空ありとして扱う);
    assert!(空ありか(方針));
    assert!(空を描くか(方針, 空の起動指定::空ありとして扱う));
}

#[test]
fn 空を描かない指定は方針を残したまま空パスだけを外す() {
    let 方針 = 世界の空方針を決める("terrain_origin", 空の起動指定::空を描かない);
    assert!(空ありか(方針));
    assert!(!空を描くか(方針, 空の起動指定::空を描かない));
}

#[test]
fn 空なしの世界は指定が無ければ空を描かない() {
    let 方針 = 世界の空方針を決める("quad", 空の起動指定::方針に従う);
    assert!(!空を描くか(方針, 空の起動指定::方針に従う));
}

/// 遠方環境の契約は大気の媒体を要るため、検収世界が空を持たないと契約の構築が型付きの失敗になる。
#[test]
fn 間接照明の検収世界は空を持つ() {
    assert!(空ありか(世界の空方針を決める("indirect_probe", 空の起動指定::方針に従う)));
}

#[test]
fn 天空の遠方環境を選ぶのは地形世界と間接照明の検収世界である() {
    for シーン名 in ["indirect_probe", "terrain_origin"] {
        assert_eq!(世界の間接照明方針を決める(シーン名), 世界の間接照明方針::天空の遠方環境, "{シーン名}");
    }
    for シーン名 in [
        "quad",
        "helmet",
        "shadow_scene",
        "fox",
        "vegetation_cull",
        "instance_all_culled",
        "material_reload",
        "multi_material_two",
        "prop_wooden_crate",
        "prop_village",
    ] {
        assert_eq!(世界の間接照明方針を決める(シーン名), 世界の間接照明方針::定数近似, "{シーン名}");
    }
}

#[test]
fn 履歴混合を選ぶのは地形世界だけである() {
    for シーン名 in ["terrain_origin", "terrain_visual"] {
        let 方式 = 世界の時間再構成方式を決める(シーン名, 時間再構成方式の起動上書き::宣言に従う);
        assert_eq!(方式, 時間再構成方式::履歴混合, "{シーン名}");
    }
    for シーン名 in ["quad", "helmet", "shadow_scene", "fox", "indirect_probe", "vegetation_cull"] {
        let 方式 = 世界の時間再構成方式を決める(シーン名, 時間再構成方式の起動上書き::宣言に従う);
        assert_eq!(方式, 時間再構成方式::使わない, "{シーン名}");
    }
}

#[test]
fn 使わないで描く指定は地形世界の宣言も落とす() {
    let 方式 = 世界の時間再構成方式を決める("terrain_origin", 時間再構成方式の起動上書き::使わないで描く);
    assert_eq!(方式, 時間再構成方式::使わない);
}
