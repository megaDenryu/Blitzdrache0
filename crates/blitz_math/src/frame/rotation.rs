//! 回転のみを表す値オブジェクト。平行移動・スケールを持たない点で`変換`と区別する。

use std::fmt;
use std::marker::PhantomData;

use glam::Quat;

pub(super) use super::rotation_error::クォータニオンエラー;
use super::space::空間;

/// `入力空間`から`出力空間`への回転(判断43: 数学DDD)。`変換`と対になる幻影型様式で、
/// 型が合うときのみTRS合成・スケルトン階層の乗算に使える。
#[repr(transparent)]
pub struct クォータニオン<入力空間, 出力空間> {
    内部: Quat,
    _入力: PhantomData<入力空間>,
    _出力: PhantomData<出力空間>,
}

impl<入力空間, 出力空間> クォータニオン<入力空間, 出力空間> {
    pub(crate) fn 内部から生成する(内部: Quat) -> Self {
        Self {
            内部,
            _入力: PhantomData,
            _出力: PhantomData,
        }
    }

    pub(crate) fn 内部クォータニオン(&self) -> Quat {
        self.内部
    }
}

impl<入力空間: 空間, 出力空間: 空間> クォータニオン<入力空間, 出力空間> {
    /// x・y・z・w成分から回転を生成する。非正規入力はアセット読込時の浮動小数点誤差を
    /// 吸収するため正規化して受け入れる。長さがゼロに近い入力のみ型付きエラーで拒否する
    /// (判断43: 検証付き生成)。
    pub fn 生成する(x: f32, y: f32, z: f32, w: f32) -> Result<Self, クォータニオンエラー> {
        let 内部 = Quat::from_xyzw(x, y, z, w);
        if 内部.length_squared() <= f32::EPSILON {
            return Err(クォータニオンエラー::ゼロ長);
        }
        Ok(Self::内部から生成する(内部.normalize()))
    }

    /// 球面線形補間。t=0で`self`、t=1で`他方`と一致する。
    pub fn slerp(&self, 他方: &Self, t: f32) -> Self {
        Self::内部から生成する(self.内部.slerp(他方.内部, t))
    }

    /// 境界向けの唯一の生値出口: x・y・z・wの順に並べた4成分。実行時形式への書き出しがこれを使う。
    /// 順を型の外へ出すのはこの1箇所だけであり、`生成する`の引数の順と対になる。
    pub fn 境界用xyzw配列(&self) -> [f32; 4] {
        self.内部.to_array()
    }
}

impl<空間種: 空間> クォータニオン<空間種, 空間種> {
    /// 回転なし(恒等)を表す。
    pub fn 恒等() -> Self {
        Self::内部から生成する(Quat::IDENTITY)
    }
}

impl<入力空間: 空間, 出力空間: 空間> クォータニオン<入力空間, 出力空間> {
    /// 入力空間の3軸が出力空間の3軸と一致する回転(回してはいない)。剛体の主軸が重心局所の軸そのものであるとき等に使う。
    pub fn 軸が一致する回転() -> Self {
        Self::内部から生成する(Quat::IDENTITY)
    }
}

// 手動実装する理由: 幻影型パラメータにClone/Copy/PartialEq境界を要求しないため。
impl<入力空間, 出力空間> Clone for クォータニオン<入力空間, 出力空間> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<入力空間, 出力空間> Copy for クォータニオン<入力空間, 出力空間> {}

impl<入力空間, 出力空間> PartialEq for クォータニオン<入力空間, 出力空間> {
    fn eq(&self, other: &Self) -> bool {
        self.内部 == other.内部
    }
}

impl<入力空間, 出力空間> fmt::Debug for クォータニオン<入力空間, 出力空間> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("クォータニオン").field("内部", &self.内部).finish()
    }
}
