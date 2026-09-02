//! 参照計算へ渡す4つの拘束のバッチの静的な引数の一覧。種類ごとの密なバッチ(判断10)の入力をひとつの値に束ね、使わない種類は空にする。

use super::connection_batch::添字付き接続拘束;
use super::point_distance_batch::添字付き点と剛体の距離拘束;
use super::target_batch::添字付き剛体の目標拘束;
use super::twist_batch::添字付きねじり拘束;

#[derive(Debug, Clone, PartialEq)]
pub struct 剛体の拘束の一覧 {
    pub 剛体の目標拘束: Vec<添字付き剛体の目標拘束>,
    pub 接続拘束: Vec<添字付き接続拘束>,
    pub ねじり拘束: Vec<添字付きねじり拘束>,
    pub 点と剛体の距離拘束: Vec<添字付き点と剛体の距離拘束>,
}

impl 剛体の拘束の一覧 {
    /// 拘束を1つも持たない一覧。自由回転の題材と、1種類だけを足す検査の土台である。
    pub fn 空() -> Self {
        Self {
            剛体の目標拘束: Vec::new(),
            接続拘束: Vec::new(),
            ねじり拘束: Vec::new(),
            点と剛体の距離拘束: Vec::new(),
        }
    }
}
