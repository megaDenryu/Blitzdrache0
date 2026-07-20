//! 姿勢: ジョイントごとのローカルTRSの列。添字はスキンのジョイント添字と対応する。

use super::joint_pose::関節TRS;

#[derive(Debug, Clone, PartialEq)]
pub struct 姿勢 {
    関節姿勢一覧: Vec<関節TRS>,
}

impl 姿勢 {
    pub fn 生成する(関節姿勢一覧: Vec<関節TRS>) -> Self {
        Self { 関節姿勢一覧 }
    }

    pub fn 関節数(&self) -> usize {
        self.関節姿勢一覧.len()
    }

    /// 不変条件: `添字`は`関節数()`未満(呼び出し元が保証する)。
    pub fn 関節(&self, 添字: usize) -> &関節TRS {
        &self.関節姿勢一覧[添字]
    }
}
