//! 1フレーム描画の入力をまとめた値オブジェクト(判断24)。
//! カメラ位置・ライティング有効フラグの追加で引数が増えたため、
//! `一フレーム描画する`系のAPIはこの1つの入力にまとめる。

use blitz_math::{クリップ, ワールド, 位置, 変換};

use crate::clear_color::クリアカラー;

#[derive(Debug, Clone, Copy)]
pub struct フレーム描画入力 {
    pub クリア色: クリアカラー,
    pub ビュー射影: 変換<ワールド, クリップ>,
    /// PBRのライティング計算(視線ベクトル)に使うカメラのワールド位置。
    pub カメラ位置: 位置<ワールド>,
    /// falseならunlit(albedo*TINTのみ)、trueならCook-Torrance GGX + Lambertの
    /// フルライティングを行う(判断26)。
    pub ライティング有効: bool,
}
