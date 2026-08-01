//! 実表示時刻計測の公開語彙。要求(呼び出し元が計測を望むか)・状況(物理デバイスが計測できるか)・
//! 観測(実際に採れた時刻)の3つで1つの計測の境界をなし、どれか1つだけでは意味を持たないため同じ器に置く。
//! `参照: _doc/設計/イベントループとフレームペーシング.md`

mod observation;
mod request;
mod status;

pub use observation::実表示観測;
pub use request::実表示計測要求;
pub use status::実表示計測状況;
