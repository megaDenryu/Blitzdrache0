//! 世界ごとに焼くアセットの宣言。担当するのは「どの安定IDへどのソースをどの種別で焼くか」だけであり、
//! 出力ルートの選択も目録の読み方も知らない。宣言をここへ集めることで、世界を1つ足すときに触る場所が1箇所に閉じる。
//! 定義1件を必須・任意・ソース専用のどれとして作るかは`definition_kind`が持つ。

use super::super::catalog::{アセット定義, ソース種別, 同居植生宣言};
use super::definition_kind::{ソース専用定義, 任意定義, 外部アセット定義, 必須定義};

/// 地形世界の起動時シーン。レンダラーはチャンクが1つも常駐しない期間にも描画対象を要求するため、束ID0を占める最小の対象が要る。
/// 板の世界の`quad`と同じソースを別IDで登録するのは、初期カメラがシーン名で決まり、地形の俯瞰視点をこのIDへ紐づけるためである。
/// 参照: `crates/blitz_app/src/app/scene_camera.rs`
const 地形世界の起動時シーン: (&str, &str) = ("terrain_origin", "smoke/quad.gltf");

/// 植生世界の原型ソース。起動時シーンもチャンクもこの1つのglTFを原型として読む。
const 植生原型ソース: &str = "vegetation_world/archetype.gltf";

/// 段を2つ持つ原型ソース。個体別LODの検収と、地形世界の同居植生がこちらを読む。
const 植生詳細段原型ソース: &str = "vegetation_world/archetype_lod.gltf";

/// 地形世界が同居させる植生の原型を指す安定ID。実行時形式は焼かず、地形チャンクが素材として読むだけである。
const 地形世界の植生原型: &str = "terrain_vegetation_archetype";

/// 地形チャンク1つが持つ個体数の既定。既存の検収はこの密度で走り、束の追加と解除で状態が生まれて消えることを見るのに足りる軽さである。
/// 物量の計測は`ow4-bench`が密度だけを変えた別の出力ルートへ焼くため、この既定は物量点の変更で動かさない。
pub(super) const 地形同居の既定個体数: usize = 64;

/// 画素判定に使う個体数。2×2の格子になり、画面を4分割した各領域へ1体ずつ描かれる構図になる。
pub(super) const 画素判定の個体数: usize = 4;

/// 計数判定で確保数の増え方を比べる相手の個体数。
pub(super) const 計数判定の個体数: usize = 64;

/// 群がカメラ視錐台にもライト視錐台にも入らないシーン。中身は`vegetation_4`と同じ4個体の群であり、名前だけが違う。
/// 名前を`vegetation`で始めないことが、既定カメラと既定の影範囲を選ばせて群を両方の視錐台から外す(理由は`xtask/src/cloth_empty.rs`)。
const 両視錐台外の群シーン: &str = "instance_all_culled";

/// Blenderが生成した小物1体の検収シーン。外部のアセットリポジトリの`props/`からglbを参照する。
/// 安定IDを`prop_`で始めることが、この世界のカメラ姿勢と読み戻しだけの検収計画を選ばせる。
/// 参照: `crates/blitz_app/src/app/scene_camera.rs`と`crates/blitz_app/src/smoke/mod.rs`
const 小物の木箱シーン: (&str, &str) = ("prop_wooden_crate", "props/wooden_crate.glb");

/// 材質を3つ持つ小物1体の検収シーン。木の本体へ鉄の帯と真鍮の角金具を組んだ箱であり、Blenderが書き出した実アセットで
/// 1メッシュ複数材質の受入を確かめる材料である。安定IDを`prop_`で始める理由は木箱と同じである。
/// 参照: `_doc/設計/マルチマテリアルと材質境界.md`「段階導入」E段
const 小物の金具付き木箱シーン: (&str, &str) = ("prop_banded_chest", "props/banded_chest.glb");

/// 材質境界の検収シーン。同じ形を2材質2プリミティブで塗るものと、1材質1プリミティブで塗る対照の2つを焼く。
/// 安定IDを`multi_material`で始めることが、書き換えもピクセル判定も持たない読み戻しだけの検収計画を選ばせる。
/// 参照: `crates/blitz_app/src/smoke/mod.rs`
const 材質境界の二材質シーン: (&str, &str) = ("multi_material_two", "smoke/multi_material_two.gltf");
const 材質境界の単一材質シーン: (&str, &str) = ("multi_material_one", "smoke/multi_material_one.gltf");

pub(super) fn 板の世界の一覧() -> Vec<アセット定義> {
    vec![
        必須定義("quad", "smoke/quad.gltf", ソース種別::Gltfシーン),
        必須定義(材質境界の二材質シーン.0, 材質境界の二材質シーン.1, ソース種別::Gltfシーン),
        必須定義(材質境界の単一材質シーン.0, 材質境界の単一材質シーン.1, ソース種別::Gltfシーン),
        必須定義("quad_alt", "smoke/quad_alt.gltf", ソース種別::Gltfシーン),
        必須定義("shadow_scene", "smoke/shadow_scene.gltf", ソース種別::Gltfシーン),
        任意定義("helmet", "samples/DamagedHelmet/DamagedHelmet.glb"),
        任意定義("fox", "samples/Fox/Fox.glb"),
        外部アセット定義(小物の木箱シーン.0, 小物の木箱シーン.1),
        外部アセット定義(小物の金具付き木箱シーン.0, 小物の金具付き木箱シーン.1),
    ]
}

pub(super) fn 地形の世界の一覧() -> Vec<アセット定義> {
    vec![
        必須定義(地形世界の起動時シーン.0, 地形世界の起動時シーン.1, ソース種別::Gltfシーン),
        ソース専用定義(地形世界の植生原型, 植生詳細段原型ソース),
    ]
}

/// 地形チャンクが焼く同居植生の宣言。地形世界のチャンクだけがこれを持つ。個体数を引数で受け取るのは、物量計測が原型・マテリアル・座標を固定したまま密度だけを変えるためである。
pub(super) fn 地形の同居植生(個体数: usize) -> 同居植生宣言 {
    同居植生宣言 {
        原型の安定id: 地形世界の植生原型,
        個体数,
    }
}

pub(super) fn 植生の世界の一覧() -> Vec<アセット定義> {
    vec![
        必須定義("vegetation_4", 植生原型ソース, 植生種別(画素判定の個体数)),
        必須定義("vegetation_64", 植生原型ソース, 植生種別(計数判定の個体数)),
        必須定義("vegetation_cull", 植生原型ソース, ソース種別::植生可視判定),
        必須定義("vegetation_single", 植生原型ソース, ソース種別::植生単一個体),
        必須定義("vegetation_lod", 植生詳細段原型ソース, ソース種別::植生詳細段),
        必須定義(両視錐台外の群シーン, 植生原型ソース, 植生種別(画素判定の個体数)),
    ]
}

pub(super) fn 植生種別(個体数: usize) -> ソース種別 {
    ソース種別::植生 { 個体数 }
}
