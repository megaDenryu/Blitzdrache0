//! アセットが持つテクスチャの安定した識別子。担当するのは、アセット側の識別と資源表世代内のGPU添字を型で分けることである。
//!
//! 注意: このIDをディスクリプタの添字として直接使ってはならない。ストリーミングでの再配置と世代の作り直しを表せなくなる。
//! ID→スロットの解決はテクスチャ台帳だけが行う。
//! 参照: `crates/blitz_render/src/vulkan/material_table/texture_registry.rs`

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct テクスチャID {
    値: u64,
}

impl テクスチャID {
    pub(crate) const fn 生成する(値: u64) -> Self {
        Self { 値 }
    }

    /// 失敗の表示と台帳の鍵で使う生値。
    pub(crate) const fn 値(self) -> u64 {
        self.値
    }
}
