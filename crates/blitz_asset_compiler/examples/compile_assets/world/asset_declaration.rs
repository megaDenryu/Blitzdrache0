//! 世界ごとに焼くアセットの宣言。担当するのは「どの安定IDへどのソースをどの種別で焼くか」だけであり、
//! 出力ルートの選択も目録の読み方も知らない。宣言をここへ集めることで、世界を1つ足すときに触る場所が1箇所に閉じる。

use super::super::catalog::{アセット定義, ソース種別};

/// 地形世界の起動時シーン。レンダラーはチャンクが1つも常駐しない期間にも描画対象を要求するため、束ID0を占める最小の対象が要る。
/// 板の世界の`quad`と同じソースを別IDで登録するのは、初期カメラがシーン名で決まり、地形の俯瞰視点をこのIDへ紐づけるためである。
/// 参照: `crates/blitz_app/src/app/scene_camera.rs`
const 地形世界の起動時シーン: (&str, &str) = ("terrain_origin", "smoke/quad.gltf");

/// 植生世界の原型ソース。起動時シーンもチャンクもこの1つのglTFを原型として読む。
const 植生原型ソース: &str = "vegetation_world/archetype.gltf";

/// 画素判定に使う個体数。2×2の格子になり、画面を4分割した各領域へ1体ずつ描かれる構図になる。
pub(super) const 画素判定の個体数: usize = 4;

/// 計数判定で確保数の増え方を比べる相手の個体数。
pub(super) const 計数判定の個体数: usize = 64;

pub(super) fn 板の世界の一覧() -> Vec<アセット定義> {
    vec![
        必須定義("quad", "smoke/quad.gltf", ソース種別::Gltfシーン),
        必須定義("quad_alt", "smoke/quad_alt.gltf", ソース種別::Gltfシーン),
        必須定義("shadow_scene", "smoke/shadow_scene.gltf", ソース種別::Gltfシーン),
        任意定義("helmet", "samples/DamagedHelmet/DamagedHelmet.glb"),
        任意定義("fox", "samples/Fox/Fox.glb"),
    ]
}

pub(super) fn 地形の世界の一覧() -> Vec<アセット定義> {
    vec![必須定義(地形世界の起動時シーン.0, 地形世界の起動時シーン.1, ソース種別::Gltfシーン)]
}

pub(super) fn 植生の世界の一覧() -> Vec<アセット定義> {
    vec![
        必須定義("vegetation_4", 植生原型ソース, 植生種別(画素判定の個体数)),
        必須定義("vegetation_64", 植生原型ソース, 植生種別(計数判定の個体数)),
        必須定義("vegetation_cull", 植生原型ソース, ソース種別::植生可視判定),
    ]
}

pub(super) fn 植生種別(個体数: usize) -> ソース種別 {
    ソース種別::植生 { 個体数 }
}

fn 必須定義(名前: &'static str, 相対パス: &'static str, 種別: ソース種別) -> アセット定義 {
    アセット定義 {
        名前,
        相対パス,
        必須: true,
        種別,
    }
}

/// 標準サンプルは`fetch-assets`で取得するまで存在しないため、無ければ飛ばす。
fn 任意定義(名前: &'static str, 相対パス: &'static str) -> アセット定義 {
    アセット定義 {
        名前,
        相対パス,
        必須: false,
        種別: ソース種別::Gltfシーン,
    }
}
