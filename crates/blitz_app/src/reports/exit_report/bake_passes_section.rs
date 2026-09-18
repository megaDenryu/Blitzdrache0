//! `--report-atmosphere-passes`の報告の節。受け取るのは終了時の観測一式、返すものは無い。担当するのは、大気のベイク済み画像と
//! 間接照明の表現を焼いたパスの本数の記録と見込み、およびそれと並べて読む天空配線の記録(空中遠近の最遠距離・遠方環境の鍵・
//! 太陽天頂区間)をこの順で出すことである。

use blitz_render::レンダラー;

use crate::app::終了時の観測一式;
use crate::reports::{aerial_farthest_distance, atmosphere_passes, distant_environment_key};

pub(super) fn 大気のベイク済み画像の生成パス数の節を出す(観測: &終了時の観測一式<'_>) {
    match 観測.レンダラー.map(レンダラー::大気のベイク済み画像生成パス数の記録を取得する) {
        Some(記録) => atmosphere_passes::大気のベイク済み画像生成パス数を表示する(記録),
        None => println!("大気のベイク済み画像生成パス数: レンダラーが生成されなかったため数えていない"),
    }
    match 観測.レンダラー {
        Some(レンダラー) => {
            atmosphere_passes::間接照明生成パス数を表示する(レンダラー.間接照明生成パス数の記録を取得する());
            atmosphere_passes::間接照明生成パス数の見込みを表示する(レンダラー.間接照明の焼き上げ本数の見込みを取得する());
        }
        None => println!("間接照明生成パス数: レンダラーが生成されなかったため数えていない"),
    }
    aerial_farthest_distance::空中遠近の最遠距離を表示する(観測.天空.空中遠近の最遠距離の記録);
    distant_environment_key::遠方環境の鍵を表示する(観測.天空.遠方環境の鍵の記録);
    distant_environment_key::太陽天頂区間を表示する(観測.天空.太陽天頂区間の記録);
}
