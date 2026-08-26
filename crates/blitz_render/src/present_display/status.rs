//! 実表示時刻計測の対応状況。非対応の理由を型で区別し、無言の計測省略を防ぐ。

/// 物理デバイスが実表示時刻の計測(VK_KHR_present_id + VK_KHR_present_wait)に対応しているか。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 実表示計測状況 {
    要求していない, // レンダラー生成時に計測を要求しなかったため、拡張を有効化していない(既定条件)
    拡張が無い,     // VK_KHR_present_id と VK_KHR_present_wait の少なくとも一方が物理デバイスに無い
    機能が無効,     // 両拡張はあるが、presentId と presentWait の少なくとも一方の機能フラグが false である
    計測できる,     // 両拡張と両機能がそろっており、提示IDの発番と表示完了の待機ができる
}

impl 実表示計測状況 {
    /// 報告文へそのまま載せる説明。
    pub fn 名称(self) -> &'static str {
        match self {
            Self::要求していない => "無効(起動時に計測を要求しなかったため拡張を有効化していない)",
            Self::拡張が無い => "非対応(VK_KHR_present_id / VK_KHR_present_wait が物理デバイスに無い)",
            Self::機能が無効 => "非対応(presentId / presentWait の機能フラグが無効)",
            Self::計測できる => "対応(提示IDと提示待機で実表示時刻を観測できる)",
        }
    }
}
