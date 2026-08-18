//! 一覧内の選択位置を表す値オブジェクト。0始まりの添字を裸のusizeのまま呼び出し側へ持ち出させず、
//! 上下移動が一覧の境界を超えないことをこの型のメソッドだけで保証する。

#[derive(Clone, Copy)]
pub(crate) struct カーソル位置 {
    位置: usize,
    件数: usize,
}

impl カーソル位置 {
    pub(crate) fn 生成する(件数: usize) -> Self {
        Self {
            位置: 0, 件数: 件数.max(1)
        }
    }

    pub(crate) fn 位置(&self) -> usize {
        self.位置
    }

    pub(crate) fn 上へ移動する(self) -> Self {
        Self {
            位置: self.位置.saturating_sub(1),
            ..self
        }
    }

    pub(crate) fn 下へ移動する(self) -> Self {
        Self {
            位置: (self.位置 + 1).min(self.件数 - 1),
            ..self
        }
    }

    pub(crate) fn 指定数だけ上へ移動する(self, 個数: usize) -> Self {
        Self {
            位置: self.位置.saturating_sub(個数),
            ..self
        }
    }

    pub(crate) fn 指定数だけ下へ移動する(self, 個数: usize) -> Self {
        Self {
            位置: (self.位置 + 個数).min(self.件数 - 1),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 先頭より上へは進まない() {
        let カーソル = カーソル位置::生成する(3).上へ移動する();
        assert_eq!(カーソル.位置(), 0);
    }

    #[test]
    fn 末尾より下へは進まない() {
        let カーソル = カーソル位置::生成する(3).下へ移動する().下へ移動する().下へ移動する();
        assert_eq!(カーソル.位置(), 2);
    }

    #[test]
    fn 指定数だけ移動して境界で止まる() {
        let カーソル = カーソル位置::生成する(10).指定数だけ下へ移動する(4);
        assert_eq!(カーソル.位置(), 4);
        let カーソル = カーソル.指定数だけ上へ移動する(100);
        assert_eq!(カーソル.位置(), 0);
    }
}
