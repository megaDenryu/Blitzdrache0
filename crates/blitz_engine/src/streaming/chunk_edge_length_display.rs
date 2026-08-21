//! チャンク一辺を診断文へ表示する外部トレイト実装。

use std::fmt::{Display, Formatter};

use super::チャンク一辺;

impl Display for チャンク一辺 {
    fn fmt(&self, 出力: &mut Formatter<'_>) -> std::fmt::Result {
        write!(出力, "{}m", self.f32値())
    }
}
