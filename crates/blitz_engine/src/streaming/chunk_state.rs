//! 非同期読込からGPU使用完了後の解除までのチャンク状態。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum チャンク状態 {
    読込待ち,
    読込中,
    準備済み,
    GPU転送中,
    フレーム反映待ち,
    常駐,
    解除待ち,
}
