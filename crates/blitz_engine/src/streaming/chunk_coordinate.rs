//! XZ平面の整数座標と、永続化に使う一意な64ビットIDの対応。

use crate::チャンクID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct チャンク座標 {
    x: i32,
    z: i32,
}

impl チャンク座標 {
    pub(super) fn 生成する(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn x(self) -> i32 {
        self.x
    }

    pub fn z(self) -> i32 {
        self.z
    }

    pub fn id(self) -> チャンクID {
        let xビット = u32::from_ne_bytes(self.x.to_ne_bytes());
        let zビット = u32::from_ne_bytes(self.z.to_ne_bytes());
        チャンクID::生成する((u64::from(xビット) << 32) | u64::from(zビット))
    }
}
