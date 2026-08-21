//! 綴りから世界の種別への見分けの検査。名前つきの世界が群の既定より先に選ばれること、群の接頭辞が効くこと、
//! どちらにも当たらない綴りが方針を1つも持たない枝へ落ちることを確かめる。

#![allow(clippy::unwrap_used)]

use super::world_kind::{世界の種別, 地形世界の種別, 小物世界の種別, 植生の検収世界の種別};
use super::起動時シーン;

fn 種別を解く(綴り: &str) -> 世界の種別 {
    起動時シーン::綴りから解析する(綴り).unwrap().種別()
}

#[test]
fn 名前つきの世界が綴りで選ばれる() {
    assert_eq!(種別を解く("quad"), 世界の種別::平面板);
    assert_eq!(種別を解く("helmet"), 世界の種別::ヘルメット);
    assert_eq!(種別を解く("shadow_scene"), 世界の種別::影の検収世界);
    assert_eq!(種別を解く("fox"), 世界の種別::キツネ);
    assert_eq!(種別を解く("material_reload"), 世界の種別::材質差し替えの検収世界);
    assert_eq!(種別を解く("indirect_probe"), 世界の種別::間接照明の検収世界);
    assert_eq!(種別を解く("instance_all_culled"), 世界の種別::両視錐台外の群の検収世界);
}

#[test]
fn 群の中の名前つきの世界が群の既定より先に選ばれる() {
    assert_eq!(種別を解く("terrain_origin"), 世界の種別::地形世界(地形世界の種別::起点));
    assert_eq!(種別を解く("terrain_visual"), 世界の種別::地形世界(地形世界の種別::目視見本));
    assert_eq!(種別を解く("terrain_fox_tour"), 世界の種別::地形世界(地形世界の種別::場所巡り));
    assert_eq!(種別を解く("terrain_editor_world"), 世界の種別::地形世界(地形世界の種別::エディター));
    assert_eq!(種別を解く("terrain_night_lights"), 世界の種別::地形世界(地形世界の種別::夜灯り));
    assert_eq!(種別を解く("prop_village"), 世界の種別::小物世界(小物世界の種別::集落));
    assert_eq!(
        種別を解く("prop_stone_hut_interior"),
        世界の種別::小物世界(小物世界の種別::石の小屋の屋内)
    );
    assert_eq!(
        種別を解く("prop_part_house_row"),
        世界の種別::小物世界(小物世界の種別::部品で組んだ家の並び)
    );
    assert_eq!(
        種別を解く("prop_part_tree_row"),
        世界の種別::小物世界(小物世界の種別::部品で組んだ木の並び)
    );
    assert_eq!(
        種別を解く("prop_part_frame_row"),
        世界の種別::小物世界(小物世界の種別::部品で組んだ一間四方の骨格の並び)
    );
}

#[test]
fn 床を同居させた植生の検収世界が3つとも名前で選ばれる() {
    for 綴り in ["vegetation_cull", "vegetation_shadow_range", "vegetation_single"] {
        assert_eq!(
            種別を解く(綴り),
            世界の種別::植生の検収世界(植生の検収世界の種別::床を同居させた),
            "{綴り}"
        );
    }
}

#[test]
fn 接頭辞の群が名前つきでない綴りを受ける() {
    assert_eq!(種別を解く("vegetation_4"), 世界の種別::植生の検収世界(植生の検収世界の種別::床を持たない));
    assert_eq!(種別を解く("prop_lantern"), 世界の種別::小物世界(小物世界の種別::名前で選ばれない小物));
    assert_eq!(種別を解く("terrain_forest"), 世界の種別::地形世界(地形世界の種別::名前で選ばれない地形));
    assert_eq!(種別を解く("multi_material_one"), 世界の種別::二材質の検収世界);
    assert_eq!(種別を解く("texture_compression"), 世界の種別::テクスチャ圧縮の検収世界);
}

#[test]
fn どの群にも当たらない綴りが方針を持たない枝へ落ちる() {
    assert_eq!(種別を解く("material_reload_alt"), 世界の種別::名前で選ばれない世界);
    assert_eq!(種別を解く("知らない世界"), 世界の種別::名前で選ばれない世界);
}

#[test]
fn 空の綴りは安定idになれず失敗する() {
    assert!(起動時シーン::綴りから解析する("").is_err());
}

#[test]
fn 既定のシーンは平面板である() {
    let 既定 = 起動時シーン::既定();
    assert_eq!(既定.種別(), 世界の種別::平面板);
    assert_eq!(既定.安定id().文字列を返す(), "quad");
}

#[test]
fn 材質差し替えの検収世界だけが自分の差し替えの対を持つ() {
    let 材質差し替え = 起動時シーン::綴りから解析する("material_reload").unwrap().差し替える生成物の対();
    assert_eq!(材質差し替え.複写元().文字列を返す(), "material_reload_alt");
    assert_eq!(材質差し替え.複写先().文字列を返す(), "material_reload");
    let 平面板 = 起動時シーン::既定().差し替える生成物の対();
    assert_eq!(平面板.複写元().文字列を返す(), "quad_alt");
    assert_eq!(平面板.複写先().文字列を返す(), "quad");
}
