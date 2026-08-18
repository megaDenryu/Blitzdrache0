//! メニューの対話中に変化する表示状態。カーソル位置・数字入力・表示窓の開始位置をまとめて持ち、
//! キー入力ごとにこの型のメソッドだけで更新する。生のusizeを自由関数の連鎖へ&mutで通さず、
//! この型が所有する状態として閉じることで、カーソル移動と表示窓の追従が常に一致することを保証する。

use super::cursor::カーソル位置;

pub(crate) struct メニュー表示状態 {
    件数: usize,
    カーソル: カーソル位置,
    表示窓開始: usize,
    入力中の番号: String,
}

impl メニュー表示状態 {
    pub(crate) fn 生成する(件数: usize) -> Self {
        Self {
            件数,
            カーソル: カーソル位置::生成する(件数),
            表示窓開始: 0,
            入力中の番号: String::new(),
        }
    }

    pub(crate) fn 上へ移動する(&mut self) {
        self.カーソル = self.カーソル.上へ移動する();
        self.入力を消す();
    }

    pub(crate) fn 下へ移動する(&mut self) {
        self.カーソル = self.カーソル.下へ移動する();
        self.入力を消す();
    }

    pub(crate) fn ページ上へ移動する(&mut self, 可視行数: usize) {
        self.カーソル = self.カーソル.指定数だけ上へ移動する(可視行数);
        self.入力を消す();
    }

    pub(crate) fn ページ下へ移動する(&mut self, 可視行数: usize) {
        self.カーソル = self.カーソル.指定数だけ下へ移動する(可視行数);
        self.入力を消す();
    }

    pub(crate) fn 数字を追加する(&mut self, 数字: char) {
        self.入力中の番号.push(数字);
    }

    pub(crate) fn 末尾の数字を消す(&mut self) {
        self.入力中の番号.pop();
    }

    pub(crate) fn 入力を消す(&mut self) {
        self.入力中の番号.clear();
    }

    /// Enterが押された時点の選択位置。数字を入力中ならその番号(1始まり)を、
    /// 何も入力していなければカーソル位置を使う。範囲外の番号は`None`にする。
    pub(crate) fn 選択位置(&self) -> Option<usize> {
        if self.入力中の番号.is_empty() {
            return Some(self.カーソル.位置());
        }
        super::selection_index::添字へ変換する(&self.入力中の番号, self.件数)
    }

    /// カーソルが表示窓の外へ出ないよう、表示窓の開始位置を追従させる。
    pub(crate) fn 表示窓を追従させる(&mut self, 可視行数: usize) {
        let 可視行数 = 可視行数.max(1);
        let カーソル位置 = self.カーソル.位置();
        if カーソル位置 < self.表示窓開始 {
            self.表示窓開始 = カーソル位置;
        } else if カーソル位置 >= self.表示窓開始 + 可視行数 {
            self.表示窓開始 = カーソル位置 + 1 - 可視行数;
        }
    }

    pub(crate) fn カーソル位置(&self) -> usize {
        self.カーソル.位置()
    }

    pub(crate) fn 表示窓開始(&self) -> usize {
        self.表示窓開始
    }

    pub(crate) fn 入力中の番号(&self) -> &str {
        &self.入力中の番号
    }
}

#[cfg(test)]
mod tests;
