//! 空間に属する3成分の量を定義する型の雛形。速度・角速度・力・回転力・衝撃・回転衝撃・回転ベクトルのように、
//! 成分が1つの単位型で表され、同じ次元どうしの加減算と符号の反転と長さだけを共通に持つ型が使う。
//! 雛形にするのは、これらの型が「どの単位の3成分か」だけで違い、幻影型の空間の扱い(deriveが空間種にも境界を要求するため手動で実装する)を
//! 型ごとに書き写すと同じ50行が7回並ぶためである。次元の合成(速度 × 秒 = 変位 等)は型ごとのファイルが持つ。

/// 成分の単位型と空間の幻影型を持つ3成分の量を定義する。`$成分`は`生成する(f32)`と`値()`を持つ単位型である。
macro_rules! 三成分の量を定義する {
    ($(#[$説明:meta])* $型:ident, $成分:ty) => {
        $(#[$説明])*
        #[repr(transparent)]
        pub struct $型<空間種> {
            内部: glam::Vec3,
            _空間: std::marker::PhantomData<空間種>,
        }

        impl<空間種: $crate::frame::space::空間> $型<空間種> {
            pub fn 成分から生成する(x: $成分, y: $成分, z: $成分) -> Self {
                Self::内部から生成する(glam::Vec3::new(x.値(), y.値(), z.値()))
            }

            /// 方向へ大きさぶんの量。大きさは負でもよく、そのときは方向の逆を向く。
            pub fn 方向へ生成する(方向: $crate::frame::direction::方向<空間種>, 大きさ: $成分) -> Self {
                Self::内部から生成する(方向.内部ベクトル() * 大きさ.値())
            }

            pub fn 零() -> Self {
                Self::内部から生成する(glam::Vec3::ZERO)
            }

            pub fn x(&self) -> $成分 {
                <$成分>::生成する(self.内部.x)
            }

            pub fn y(&self) -> $成分 {
                <$成分>::生成する(self.内部.y)
            }

            pub fn z(&self) -> $成分 {
                <$成分>::生成する(self.内部.z)
            }

            pub fn 長さ(&self) -> $成分 {
                <$成分>::生成する(self.内部.length())
            }

            /// 方向へ射影した成分。方向の向きが正、逆が負である。
            pub fn 方向に沿う成分(&self, 方向: $crate::frame::direction::方向<空間種>) -> $成分 {
                <$成分>::生成する(self.内部.dot(方向.内部ベクトル()))
            }

            pub fn 有限か(&self) -> bool {
                self.内部.is_finite()
            }

            pub(crate) fn 内部ベクトル(&self) -> glam::Vec3 {
                self.内部
            }

            pub(crate) fn 内部から生成する(内部: glam::Vec3) -> Self {
                Self { 内部, _空間: std::marker::PhantomData }
            }
        }

        impl<空間種: $crate::frame::space::空間> std::ops::Add for $型<空間種> {
            type Output = Self;
            fn add(self, 右辺: Self) -> Self {
                Self::内部から生成する(self.内部 + 右辺.内部)
            }
        }

        impl<空間種: $crate::frame::space::空間> std::ops::Sub for $型<空間種> {
            type Output = Self;
            fn sub(self, 右辺: Self) -> Self {
                Self::内部から生成する(self.内部 - 右辺.内部)
            }
        }

        impl<空間種: $crate::frame::space::空間> std::ops::Neg for $型<空間種> {
            type Output = Self;
            fn neg(self) -> Self {
                Self::内部から生成する(-self.内部)
            }
        }

        impl<空間種> Clone for $型<空間種> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<空間種> Copy for $型<空間種> {}

        impl<空間種> PartialEq for $型<空間種> {
            fn eq(&self, 相手: &Self) -> bool {
                self.内部 == 相手.内部
            }
        }

        impl<空間種> std::fmt::Debug for $型<空間種> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($型)).field("内部", &self.内部).finish()
            }
        }
    };
}

pub(super) use 三成分の量を定義する;
