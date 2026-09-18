//! 構造変更の反映の結果: 反映が適用した件数と、失効した個体へ向いていて捨てた件数。捨てた件数を数えて答えるのは、
//! 失効した個体への追加と削除を正常として受け付けつつ(契約21)、黙って捨てる形を禁じるためである。

/// 1回の反映が命令の種類ごとに適用した件数と、捨てた件数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct 構造変更の反映の結果 {
    反映した生成の件数: usize,
    反映した追加の件数: usize,
    反映した削除の件数: usize,
    反映した破棄の件数: usize,
    捨てた件数: usize,
}

impl 構造変更の反映の結果 {
    pub(crate) fn 生成を数える(&mut self) {
        self.反映した生成の件数 += 1;
    }

    pub(crate) fn 追加を数える(&mut self) {
        self.反映した追加の件数 += 1;
    }

    pub(crate) fn 削除を数える(&mut self) {
        self.反映した削除の件数 += 1;
    }

    pub(crate) fn 破棄を数える(&mut self) {
        self.反映した破棄の件数 += 1;
    }

    pub(crate) fn 捨てたものを数える(&mut self) {
        self.捨てた件数 += 1;
    }

    /// 反映待ちから生存へ移した個体の数。
    pub fn 反映した生成の件数(&self) -> usize {
        self.反映した生成の件数
    }

    /// 置き場へ入れた個体構成要素の数。
    pub fn 反映した追加の件数(&self) -> usize {
        self.反映した追加の件数
    }

    /// 置き場から取り除いた個体構成要素の数。
    pub fn 反映した削除の件数(&self) -> usize {
        self.反映した削除の件数
    }

    /// 失効させて全個体構成要素を取り除いた個体の数。
    pub fn 反映した破棄の件数(&self) -> usize {
        self.反映した破棄の件数
    }

    /// 生存していない個体へ向いていた命令と、持っていない型への削除の数。反映は黙って捨てず、ここで数える。
    pub fn 捨てた件数(&self) -> usize {
        self.捨てた件数
    }
}
