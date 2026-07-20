//! 単位型: 長さ・時間・角度・速度を型で区別し、次元の合わない演算を防ぐ。
//! 参照: `_doc/計画/ユビキタス言語.md`「単位型」。

mod meter;
mod meter_per_second;
mod radian;
mod second;

pub use meter::メートル;
pub use meter_per_second::メートル毎秒;
pub use radian::ラジアン;
pub use second::秒;
