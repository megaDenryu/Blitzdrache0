//! 読み戻しだけの検収計画を選ぶシーン名の台帳と、その判定。担当するのは名前の一覧を1箇所へ持ち、
//! 渡されたシーン名がそこへ当たるかを答えることだけである。どの計画をいつ選ぶかは親モジュールが持つ。
//!
//! 台帳を親から分けるのは、検収の世界が増えるたびにこの一覧だけが伸び、計画の分岐と押し合うためである。

/// 書き換えもピクセル判定も持たず、判定を検収側のxtaskが読み戻し画像と計数で行うシーンの名前の接頭辞。
/// 判定する入口は順に`instance-*`・`prop-draw`と`village-draw`・`multi-material-draw`・`indirect-probe`・`terrain-visual`・`texture-compression`である。
/// 既定の計画は`quad`のホットリロード検証であり、監視対象のシェーダーとquadの実行時形式を書き換えるため、
/// これらをその計画へ落とすとリポジトリのシェーダーと既存の生成物が書き換わる。
const 接頭辞一覧: [&str; 6] = [
    "vegetation",
    "prop_",
    "multi_material",
    "indirect_probe",
    "terrain_visual",
    "texture_compression",
];

/// 群が両方の視錐台から外れる検収シーン(`cargo xtask cloth-empty`)。中身は植生の検収シーンと同じだが、
/// 既定カメラと既定の影範囲を選ばせるために名前を接頭辞から外してあるため、ここで個別に同じ計画へ振る。
const 両視錐台外の群シーン: &str = "instance_all_culled";

pub(super) fn 読み戻しだけの検収シーンか(シーン名: &str) -> bool {
    シーン名 == 両視錐台外の群シーン || 接頭辞一覧.iter().any(|接頭辞| シーン名.starts_with(接頭辞))
}
