//! 1つのディスクリプタセットの中で束縛先を選ぶ番号。シェーダーの`[[vk::binding(N, S)]]`のNと同じ値である。
//!
//! 数のまま持ち回らないのは、セット番号・要素の件数・配列の添字がどれも同じ`u32`であり、取り違えても
//! 署名が通るためである。生値へ戻すのは、Vulkanの束縛の宣言と書き込み記述子を組む境界だけである。
//!
//! 番号の正本はシェーダーの宣言であり、CPU側の宣言との一致は`cargo xtask conform`が機械で見る
//! (参照: `xtask/src/conform/shader_binding.rs`)。

/// 束縛番号。定数として書けるように`const fn`で作る。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub(crate) struct 束縛番号(u32);

impl 束縛番号 {
    pub(crate) const fn 生成する(番号: u32) -> Self {
        Self(番号)
    }

    /// Vulkanの束縛の宣言と書き込み記述子へ渡す境界。
    pub(crate) fn gpu境界値(self) -> u32 {
        self.0
    }
}
