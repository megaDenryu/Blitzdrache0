//! 四方向、段差0・1・最大、細粗入替から24個の決定的な検査条件を列挙する。

pub(super) struct 検査条件 {
    pub(super) 名前: String,
    pub(super) 一方: (i32, i32),
    pub(super) 他方: (i32, i32),
    pub(super) 一方段: u8,
    pub(super) 他方段: u8,
}

const 方向一覧: [(&str, (i32, i32)); 4] = [("east", (1, 0)), ("west", (-1, 0)), ("north", (0, -1)), ("south", (0, 1))];
const 段差一覧: [u8; 3] = [0, 1, 4];

pub(super) fn 全条件() -> Vec<検査条件> {
    let mut 一覧 = Vec::with_capacity(24);
    for (方向名, 他方) in 方向一覧 {
        for 段差 in 段差一覧 {
            for 入替 in [false, true] {
                let (一方段, 他方段) = if 入替 { (段差, 0) } else { (0, 段差) };
                一覧.push(検査条件 {
                    名前: format!("{方向名}_diff{段差}_swap{}", u8::from(入替)),
                    一方: (0, 0),
                    他方,
                    一方段,
                    他方段,
                });
            }
        }
    }
    一覧
}
