//! 単位型: 長さ・面積・時間・角度・速度・質量・慣性と逆慣性・力と衝撃の大きさを型で区別し、次元の合わない演算を防ぐ。
//! 参照: `_doc/計画/ユビキタス言語.md`「単位型」。

mod global_meter;
mod global_second;
mod inverse_kilogram;
mod inverse_kilogram_square_meter;
mod inverse_meter;
mod kilogram;
mod kilogram_meter;
mod kilogram_square_meter;
mod mass_error;
mod meter;
mod meter_per_second;
mod meter_per_second_squared;
mod narrowing;
mod newton;
mod newton_meter;
mod newton_meter_second;
mod newton_second;
mod radian;
mod radian_per_second;
mod second;
mod square_meter;
mod unit_conversion_error;

pub use global_meter::大域メートル;
pub use global_second::大域秒;
pub use inverse_kilogram::逆キログラム;
pub use inverse_kilogram_square_meter::逆キログラム平方メートル;
pub use inverse_meter::逆メートル;
pub use kilogram::キログラム;
pub use kilogram_meter::キログラムメートル;
pub use kilogram_square_meter::キログラム平方メートル;
pub use mass_error::質量エラー;
pub use meter::メートル;
pub use meter_per_second::メートル毎秒;
pub use meter_per_second_squared::メートル毎秒毎秒;
pub use narrowing::倍精度の無次元量を単精度へ狭める;
pub use newton::ニュートン;
pub use newton_meter::ニュートンメートル;
pub use newton_meter_second::ニュートンメートル秒;
pub use newton_second::ニュートン秒;
pub use radian::ラジアン;
pub use radian_per_second::ラジアン毎秒;
pub use second::秒;
pub use square_meter::平方メートル;
pub use unit_conversion_error::単位変換エラー;
