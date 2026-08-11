//! glTFと画像を検証済みのエンジン内シーンへ変換する開発時アセットコンパイラ。
//! 高さ格子から地形メッシュを焼く経路も持ち、どちらも実行時形式の同じ通常メッシュへ落とす。

#![forbid(unsafe_code)]

mod asset_layout;
mod chunk_directory_source;
mod compile;
mod edge_agreement;
mod error;
mod fixed_placement;
mod generation_ledger;
mod height_field;
mod height_grid;
mod loader;
mod terrain;
mod texture_storage;
mod uniform_scale;
mod vegetation;
mod village;
mod visual_sample;

pub use asset_layout::{
    ソースルート, チャンク目録ソースの置き場, 世界のソースディレクトリ, 場所巡りの世界のディレクトリ名, 実行時形式の出力ルート
};
pub use chunk_directory_source::{チャンク目録ソースを読み込む, チャンク目録ソース項目};
pub use compile::{
    コンパイル済みシーン, ソースシーンをコンパイルする, 寸法を合わせる倍率を与えてソースシーンをコンパイルする
};
pub use edge_agreement::{ソースの高さ格子の重なり帯が一致することを確かめる, 縁の一致エラー};
pub use error::アセットコンパイルエラー;
pub use fixed_placement::{固定物の据え付け, 固定物を据えた地形チャンクをコンパイルする};
pub use generation_ledger::{
    チャンクの焼き直し判定, マップ生成の乱数の種, 今回の宣言と依存一式の内容ハッシュを求める, 内容ハッシュ, 焼き直しの勘定, 生成の出力ルート,
    生成台帳, 生成台帳の見出し, 生成台帳エラー, 種の由来,
};
pub use height_field::{
    コンパイル済み高さ場, チャンクごとの高さ格子から高さ場を組み立てる, 高さ場アセットをコンパイルする, 高さ場コンパイルエラー
};
pub use height_grid::{
    高さ格子, 高さ格子を切り出す, 高さ格子を格納する, 高さ格子を読み込む, 高さ格子エラー, 高さ格子諸元
};
pub use loader::{
    ソースシーンを読み込む, 入力契約を検査する, 原型ソース, 原型ソースを読み込む, 契約指摘, 契約検査概要, 契約検査結果, 重大度
};
pub use terrain::地形チャンクをコンパイルする;
pub use texture_storage::{
    rgba8の縮小段をbc1のバイト列へ符号化する, srgbの色として縦横を半分に縮める, srgbの色として縮小段の連なりを作る,
    srgbの色の全段をbc1のバイト列へ符号化する, テクスチャ格納方針, 方針と役割に従って原寸を格納済みテクスチャへ焼く, 材質テクスチャ役割,
};
pub use uniform_scale::寸法を合わせる一様倍率;
pub use vegetation::{
    同居植生の指定, 地形同居の群, 地形同居の群を作る, 植生チャンクをコンパイルする, 植生単一個体シーンをコンパイルする,
    植生可視判定シーンをコンパイルする, 植生影視距離シーンをコンパイルする, 植生詳細段シーンをコンパイルする,
};
pub use village::{小物群の指定, 見本の集落チャンクをコンパイルする, 配置様式};
pub use visual_sample::{目視見本の指定, 目視見本チャンクをコンパイルする};
