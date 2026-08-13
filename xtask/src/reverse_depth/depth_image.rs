//! D32の並びを読み、標準Zと逆Zを同じビュー空間の奥行きへ戻す。

use super::error::逆Z検収エラー;

pub(super) const 近クリップ: f32 = 0.1;
pub(super) const 遠クリップ: f32 = 10000.0;

pub(super) struct 深度画像 {
    深度: Vec<f32>,
}

impl 深度画像 {
    pub(super) fn バイト列から読む(バイト列: &[u8], 期待画素数: usize) -> Result<Self, 逆Z検収エラー> {
        let mut 塊一覧 = バイト列.chunks_exact(4);
        let mut 深度 = Vec::with_capacity(塊一覧.len());
        for 塊 in &mut 塊一覧 {
            深度.push(f32::from_le_bytes([塊[0], 塊[1], 塊[2], 塊[3]]));
        }
        if !塊一覧.remainder().is_empty() {
            return Err(逆Z検収エラー::深度のバイト長が4の倍数でない(バイト列.len()));
        }
        if 深度.len() != 期待画素数 {
            return Err(逆Z検収エラー::深度の画素数が違う {
                期待: 期待画素数,
                実際: 深度.len(),
            });
        }
        Ok(Self { 深度 })
    }

    pub(super) fn 深度列(&self) -> &[f32] {
        &self.深度
    }
}

pub(super) fn 標準深度の奥行き(深度: f32) -> f64 {
    let 近 = f64::from(近クリップ);
    let 遠 = f64::from(遠クリップ);
    遠 * 近 / (遠 - f64::from(深度) * (遠 - 近))
}

pub(super) fn 逆向き深度の奥行き(深度: f32) -> f64 {
    let 近 = f64::from(近クリップ);
    let 遠 = f64::from(遠クリップ);
    遠 * 近 / (近 + f64::from(深度) * (遠 - 近))
}

pub(super) fn 標準投影の深度(距離: f32) -> f32 {
    let 係数 = 遠クリップ / (近クリップ - 遠クリップ);
    let 定数 = 近クリップ * 遠クリップ / (近クリップ - 遠クリップ);
    (係数 * -距離 + 定数) / 距離
}

pub(super) fn 逆向き投影の深度(距離: f32) -> f32 {
    let 係数 = 近クリップ / (遠クリップ - 近クリップ);
    let 定数 = 遠クリップ * 近クリップ / (遠クリップ - 近クリップ);
    (係数 * -距離 + 定数) / 距離
}

#[cfg(test)]
mod tests {
    use super::{標準深度の奥行き, 近クリップ, 逆向き深度の奥行き, 遠クリップ};

    #[test]
    fn 端点の近面と遠面が逆になる() {
        assert!((標準深度の奥行き(0.0) - f64::from(近クリップ)).abs() < 1.0e-12);
        assert!((標準深度の奥行き(1.0) - f64::from(遠クリップ)).abs() < 1.0e-6);
        assert!((逆向き深度の奥行き(1.0) - f64::from(近クリップ)).abs() < 1.0e-12);
        assert!((逆向き深度の奥行き(0.0) - f64::from(遠クリップ)).abs() < 1.0e-6);
    }

    #[test]
    fn 反転した深度は同じ奥行きへ戻る() {
        for 標準深度 in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let 差 = (標準深度の奥行き(標準深度) - 逆向き深度の奥行き(1.0 - 標準深度)).abs();
            assert!(差 < 1.0e-6, "深度{標準深度}の奥行き差{差}");
        }
    }
}
