//! パイプライン生成入力のうち、描画先が決める部分を一意にした値。担当するのは、色と深度の形式と標本数という
//! 「同じでなければ同じVkPipelineを使い回せない」3つを1つの値へ閉じることである。
//!
//! 注意: スワップチェーンを作り直して実際の形式が変わったのに旧いパイプラインを返すと、dynamic renderingの
//! 描画先とパイプラインの宣言が食い違う。形式をキーの成分にすることが、その取り違えを型で防ぐ手段である。
//! 深度のみのパスは色を持たないため、色の形式は`Option`で表す(「無し」をマジックな形式値で表さない)。

use ash::vk;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct 描画先の一意化 {
    カラー形式: Option<vk::Format>,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
}

impl 描画先の一意化 {
    pub(crate) const fn 色と深度へ描く(カラー形式: vk::Format, 深度形式: vk::Format, 標本数: vk::SampleCountFlags) -> Self {
        Self {
            カラー形式: Some(カラー形式),
            深度形式,
            標本数,
        }
    }

    pub(crate) const fn 深度だけへ描く(深度形式: vk::Format, 標本数: vk::SampleCountFlags) -> Self {
        Self {
            カラー形式: None,
            深度形式,
            標本数,
        }
    }

    pub(crate) const fn カラー形式(self) -> Option<vk::Format> {
        self.カラー形式
    }

    pub(crate) const fn 深度形式(self) -> vk::Format {
        self.深度形式
    }

    pub(crate) const fn 標本数(self) -> vk::SampleCountFlags {
        self.標本数
    }
}
