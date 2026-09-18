//! 天空配線の終了時の観測。検収が終了時に読む4つ(天空状態・遠方環境の鍵の記録・空中遠近の最遠距離の記録・
//! 太陽天頂区間の記録)を1回だけ借用で束ねた値である。束ねる工程は`bake_input`が持つ(記録を持つフィールドを触るのがそこだからである)。

use blitz_engine::sky::天空状態;

use super::super::aerial_distance_record::空中遠近の最遠距離の記録;
use super::super::distant_environment_key_record::遠方環境の鍵の記録;
use super::super::sun_zenith_interval_record::太陽天頂区間の記録;

/// 終了時の報告が読む、天空配線の観測。
pub(crate) struct 天空の終了時の観測<'a> {
    pub(crate) 天空状態: Option<&'a 天空状態>,                         // 空の方針を持たない世界では無い
    pub(crate) 遠方環境の鍵の記録: &'a 遠方環境の鍵の記録,             // 太陽円盤の明るさを変えた対で鍵と判断が動かないことを検収が読む
    pub(crate) 空中遠近の最遠距離の記録: &'a 空中遠近の最遠距離の記録, // カメラの遠クリップと一致することを検収が読む
    pub(crate) 太陽天頂区間の記録: &'a 太陽天頂区間の記録,             // 焼き直したフレーム数が区間の変化回数と1対1で対応することを検収が読む
}
