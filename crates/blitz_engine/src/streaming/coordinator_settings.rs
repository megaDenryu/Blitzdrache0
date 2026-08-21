//! ストリーミング調停の起動時設定。先読み範囲と、保持してよいRAM・VRAMの上限を一組で渡す。

use super::loader_settings::チャンク読込設定;
use super::memory_amount::ストリーミングメモリ量;

#[derive(Debug, Clone, Copy)]
pub struct ストリーミング調停設定 {
    /// 所属チャンクからのチェビシェフ距離で表す先読み範囲。0は所属チャンクだけを指す。
    pub 先読み半径: u8,
    pub 上限: ストリーミングメモリ量,
    pub 読込: チャンク読込設定,
}
