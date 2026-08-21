//! 突き合わせる定数の組の型と、領域ごとの台帳の束ね。担当するのは「どの領域の台帳が検査の対象か」だけである。
//! 検査の手順は親モジュールが持ち、どの宣言とどの宣言が同じ値でなければならないかは各領域の台帳が持つ。
//! 領域で分けるのは、定数を足すときに触るのがその領域の台帳1つになるようにするためである。

mod atmosphere;
mod auto_exposure;
mod bloom;
mod building_outline_catalog;
mod clustered_lighting;
mod distant_environment;
mod editor_chunk_source;
mod instance_transform;
mod point_light_shadow;
mod scene;
mod temporal_reconstruction;
mod workgroup_threads;

/// 突き合わせる定数の組。正本の行と写しの行を、それぞれ宣言の前置きで見つける。
pub(super) struct 定数の組 {
    pub(super) 正本パス: &'static str,
    pub(super) 正本の前置き: &'static str,
    pub(super) 写しパス: &'static str,
    pub(super) 写しの前置き: &'static str,
}

/// 領域ごとの台帳。並びは検査の順にだけ効く。
pub(super) const 領域一覧: [&[定数の組]; 12] = [
    &atmosphere::定数一覧,
    &auto_exposure::定数一覧,
    &bloom::定数一覧,
    &building_outline_catalog::定数一覧,
    &clustered_lighting::定数一覧,
    &distant_environment::定数一覧,
    &editor_chunk_source::定数一覧,
    &instance_transform::定数一覧,
    &point_light_shadow::定数一覧,
    &scene::定数一覧,
    &temporal_reconstruction::定数一覧,
    &workgroup_threads::定数一覧,
];
