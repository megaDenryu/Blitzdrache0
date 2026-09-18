//! 検収の観測の終了時の統計。担当するのは、実行を通して積み上がった観測をプロセスの終わりに1回だけ集計し、
//! 報告が読む形で運ぶことだけである。呼ばれるのは終了時の1回であり、毎フレームの記録とは呼び出し頻度が違う。

use super::検収の観測;
use crate::app::frame_timing::フレーム時間統計;

/// ディスクから実行時シーンを読んだ回数。総数と、そのうち1フレーム目以降に起きた回数を対で持つ。
#[derive(Debug, Clone, Copy)]
pub(crate) struct シーン読込回数 {
    pub(crate) 総数: u64,
    pub(crate) 起動後: u64,
}

/// 終了時の報告が読む、検収の観測の統計。
pub(crate) struct 検収の観測の統計 {
    pub(crate) フレーム時間統計: Option<フレーム時間統計>, // `--report-frame-times`が指定され、ウォームアップ後のフレームがあれば`Some`
    pub(crate) 選別の区間統計: Option<フレーム時間統計>,   // `--report-instance-sections`が指定され、ウォームアップ後のフレームがあれば`Some`
    pub(crate) フレーム時間報告が必要か: bool,             // `--report-frame-times`が指定されたか
    pub(crate) インスタンス区間報告が必要か: bool,         // `--report-instance-sections`が指定されたか
    pub(crate) シーン読込回数: シーン読込回数,
}

impl 検収の観測 {
    pub(in crate::app) fn 終了時の統計を作る(&self) -> 検収の観測の統計 {
        検収の観測の統計 {
            フレーム時間統計: self.フレーム間隔計測.as_ref().and_then(crate::app::frame_timing::フレーム間隔計測::集計する),
            選別の区間統計: self.可視個体の選別の計測.as_ref().and_then(super::section_timing::区間計測::集計する),
            フレーム時間報告が必要か: self.フレーム間隔を測るか(),
            インスタンス区間報告が必要か: self.選別の時間を測るか(),
            シーン読込回数: シーン読込回数 {
                総数: self.シーン読込計数.総数(),
                起動後: self.シーン読込計数.起動後(),
            },
        }
    }
}
