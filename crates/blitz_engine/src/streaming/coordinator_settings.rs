//! ストリーミング調停の起動時設定。先読み範囲と、保持してよいRAM・VRAMの上限を一組で渡す。

use super::loader_settings::チャンク読込設定;
use super::memory_amount::ストリーミングメモリ量;

#[derive(Debug, Clone, Copy)]
pub struct ストリーミング調停設定 {
    pub 先読み半径: u8, // 所属チャンクからのチェビシェフ距離。0は所属チャンクだけ
    pub 上限: ストリーミングメモリ量,
    pub 読込: チャンク読込設定,
}
