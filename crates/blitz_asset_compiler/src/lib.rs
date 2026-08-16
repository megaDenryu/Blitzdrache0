//! glTFと画像を検証済みのエンジン内シーンへ変換する開発時アセットコンパイラ。
//! 高さ格子から地形メッシュを焼く経路も持ち、どちらも実行時形式の同じ通常メッシュへ落とす。

#![forbid(unsafe_code)]

mod assembled_scene;
mod assembly_reference;
mod asset_layout;
mod chunk_directory_source;
mod compile;
mod distant_terrain;
mod distant_terrain_bake;
mod edge_agreement;
mod error;
mod fixed_placement;
mod generation_ledger;
mod height_field;
mod height_grid;
mod lattice_noise;
mod loader;
mod nonnegative_floor;
mod part_row;
mod placed_instance_count;
mod placement;
mod scatter_bake;
mod scene_compiler;
mod terrain;
mod texture_storage;
mod uniform_scale;
mod vegetation;
mod village;
mod visual_sample;

pub use assembly_reference::{正解の姿勢, 正解表, 正解表の読み込みエラー, 組み立ての正解表のファイル};
pub use asset_layout::{
    アセット配置エラー, ソースルート, チャンクの高さ格子, チャンクの高さ格子ソース, チャンク目録ソースの置き場, 世界のディレクトリ名, 世界の広がり,
    世界の広がりエラー, 場所巡りの世界のソースディレクトリ, 実行時形式の出力ルート, 高さ格子のファイル,
};
pub use chunk_directory_source::{チャンク目録ソースを読み込む, チャンク目録ソース項目};
pub use compile::コンパイル済みシーン;
pub use distant_terrain::{
    コンパイル済み遠景, 遠景の沈み統計, 遠景アセットをコンパイルする, 遠景コンパイルエラー
};
pub use distant_terrain_bake::遠景地形の焼き方;
pub use edge_agreement::{ソースの高さ格子の重なり帯が一致することを確かめる, 縁の一致エラー};
pub use error::アセットコンパイルエラー;
pub use fixed_placement::固定物の据え付け;
pub use generation_ledger::{
    チャンクの焼き直し判定, マップ生成の乱数の種, 今回の宣言と依存一式の内容ハッシュを求める, 内容ハッシュ, 焼き直しの勘定, 生成の出力ルート,
    生成台帳, 生成台帳の見出し, 生成台帳の読み込み結果, 生成台帳エラー, 種の由来,
};
pub use height_field::{
    コンパイル済み高さ場, チャンクごとの高さ格子から高さ場を組み立てる, 高さ場アセットをコンパイルする, 高さ場コンパイルエラー
};
pub use height_grid::{
    高さ格子, 高さ格子を切り出す, 高さ格子を格納する, 高さ格子を読み込む, 高さ格子エラー, 高さ格子諸元
};
pub use lattice_noise::格子雑音の値;
pub use loader::{
    入力契約を検査するglTFのファイル, 原型ソース, 契約指摘, 契約検査概要, 契約検査結果, 接合点の読み取りの成否, 接合点読み取りエラー, 検査する契約,
    部品のglTFのファイル, 部品の接合点の読み取り, 部品の読み取りエラー, 部品カタログの読み込みエラー, 部品カタログの読み込み係, 重大度,
};
pub use part_row::{部品で組んだ並びのコンパイルエラー, 部品で組んだ並びの件数, 部品で組んだ並びの指定};
pub use placed_instance_count::置いた個体の数;
pub use placement::{
    個数を決める並べ方, 傾きの好み, 出現割合の範囲, 原型と置き方の指定, 地表への据え方, 大きさの範囲, 密度場の指定, 密度場の散布, 配置の種, 配置様式,
};
pub use scatter_bake::散布の焼き方;
pub use scene_compiler::ソースアセットのコンパイル係;
pub use texture_storage::{
    rgba8の縮小段をbc1のバイト列へ符号化する, srgbの色として縦横を半分に縮める, srgbの色として縮小段の連なりを作る,
    srgbの色の全段をbc1のバイト列へ符号化する, テクスチャ格納方針, 方針と役割に従って原寸を格納済みテクスチャへ焼く, 材質テクスチャ役割,
};
pub use uniform_scale::寸法を合わせる一様倍率;
pub use vegetation::同居植生の指定;
pub use visual_sample::目視見本の指定;
