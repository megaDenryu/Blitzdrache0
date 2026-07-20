//! ash::vk::Result を公開APIへ露出せず表現するための値オブジェクト。
//!
//! 注意: `生成する` は pub(crate) であり、ash型を「受け取って自前表現に変換する」
//! ことしかしない。公開APIからash型を組み立てたり取り出したりする経路は存在しない。

use std::fmt;

/// Vulkan呼び出し失敗時のエラーコードを、ash型を経由せず表現する値オブジェクト。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vulkan失敗コード {
    コード番号: i32,
    コード名: String,
}

impl Vulkan失敗コード {
    pub(crate) fn 生成する(結果: ash::vk::Result) -> Self {
        Self {
            コード番号: 結果.as_raw(),
            コード名: 結果.to_string(),
        }
    }

    /// Vulkan仕様上のVkResult数値表現。
    pub fn コード番号(&self) -> i32 {
        self.コード番号
    }
}

impl fmt::Display for Vulkan失敗コード {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(コード{})", self.コード名, self.コード番号)
    }
}
