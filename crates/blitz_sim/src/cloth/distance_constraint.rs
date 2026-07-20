//! 距離拘束: 2粒子間の静止距離を保つ拘束(構造+せん断。曲げ拘束は対象外。判断52)。

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 距離拘束 {
    pub 粒子a添字: u32,
    pub 粒子b添字: u32,
    pub 静止長: f32,
}
