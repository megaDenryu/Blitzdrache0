//! 間接照明の供給元と時間再構成方式の検査。天空の遠方環境を選ぶ世界が地形世界と間接照明の検収世界に
//! 限られることと、履歴混合を選ぶ世界が地形世界だけであることを固定する。空の方針の固定は親が持つ。
//!
//! 空の方針と分けるのは、背景を描くかどうかと表面へ届く照明の供給元が独立の軸であり、片方の変更が
//! もう片方の検査を巻き込まないことをファイルの分かれ目で示すためである。

use blitz_engine::sky::世界の間接照明方針;
use blitz_engine::temporal_reconstruction::時間再構成方式;

use super::{scene_policy, 種別を解く};
use crate::cli::temporal_reconstruction_settings::時間再構成方式の起動上書き;

/// 定数近似のままである世界の綴り。天空の遠方環境を選ぶ2つの世界の裏返しである。
const 定数近似の世界の綴り一覧: [&str; 10] = [
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
];

fn 間接照明方針(綴り: &str) -> 世界の間接照明方針 {
    scene_policy::世界の間接照明方針を決める(種別を解く(綴り))
}

fn 宣言に従う時間再構成方式(綴り: &str) -> 時間再構成方式 {
    scene_policy::世界の時間再構成方式を決める(種別を解く(綴り), 時間再構成方式の起動上書き::宣言に従う)
}

#[test]
fn 天空の遠方環境を選ぶのは地形世界と間接照明の検収世界である() {
    for 綴り in ["indirect_probe", "terrain_origin"] {
        assert_eq!(間接照明方針(綴り), 世界の間接照明方針::天空の遠方環境, "{綴り}");
    }
    for 綴り in 定数近似の世界の綴り一覧 {
        assert_eq!(間接照明方針(綴り), 世界の間接照明方針::定数近似, "{綴り}");
    }
}

#[test]
fn 履歴混合を選ぶのは地形世界だけである() {
    for 綴り in ["terrain_origin", "terrain_visual"] {
        assert_eq!(宣言に従う時間再構成方式(綴り), 時間再構成方式::履歴混合, "{綴り}");
    }
    for 綴り in ["quad", "helmet", "shadow_scene", "fox", "indirect_probe", "vegetation_cull"] {
        assert_eq!(宣言に従う時間再構成方式(綴り), 時間再構成方式::使わない, "{綴り}");
    }
}

#[test]
fn 使わないで描く指定は地形世界の宣言も落とす() {
    let 方式 = scene_policy::世界の時間再構成方式を決める(種別を解く("terrain_origin"), 時間再構成方式の起動上書き::使わないで描く);
    assert_eq!(方式, 時間再構成方式::使わない);
}
