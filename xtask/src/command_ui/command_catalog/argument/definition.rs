//! コマンドが受け取る引数1件の定義。担当するのは、引数の形と説明と省略したときの扱いを1つの値に保ち、
//! 形ごとに違う材料(名前の綴り・見出し・選べる綴りの一覧)をその形の枝だけに持たせることである。
//!
//! 形を判別共用体で表すのは、尋ね方もコマンド行への写し方も形ごとに違い、混ぜると意味を成さないためである。
//! 尋ねる側は網羅的な`match`で全部の形を扱うため、形を1つ足すとフォームの取りこぼしがコンパイルで落ちる。

use super::choice::選択肢;
use super::omission::省略したときの扱い;

#[derive(Clone, Copy)]
pub(crate) enum 引数定義 {
    有無だけの旗 {
        綴り: &'static str,
        説明: &'static str,
    },
    名前に続けて渡す値 {
        綴り: &'static str,
        説明: &'static str,
        扱い: 省略したときの扱い,
    },
    綴りから1つ選ぶ値 {
        名前: Option<&'static str>,
        見出し: &'static str,
        説明: &'static str,
        扱い: 省略したときの扱い,
        選択肢一覧: &'static [選択肢],
    },
    位置で渡す値 {
        見出し: &'static str,
        説明: &'static str,
        扱い: 省略したときの扱い,
    },
    位置で何個でも渡す値 {
        見出し: &'static str,
        説明: &'static str,
        扱い: 省略したときの扱い,
    },
    そのまま子へ渡す残りの語 {
        見出し: &'static str,
        説明: &'static str,
    },
}

impl 引数定義 {
    pub(crate) const fn 有無だけの旗を定義する(綴り: &'static str, 説明: &'static str) -> Self {
        Self::有無だけの旗 { 綴り, 説明 }
    }

    pub(crate) const fn 名前に続けて渡す値を定義する(
        綴り: &'static str, 説明: &'static str, 扱い: 省略したときの扱い
    ) -> Self {
        Self::名前に続けて渡す値 { 綴り, 説明, 扱い }
    }

    pub(crate) const fn 綴りから1つ選ぶ値を定義する(
        名前: Option<&'static str>,
        見出し: &'static str,
        説明: &'static str,
        扱い: 省略したときの扱い,
        選択肢一覧: &'static [選択肢],
    ) -> Self {
        Self::綴りから1つ選ぶ値 {
            名前,
            見出し,
            説明,
            扱い,
            選択肢一覧,
        }
    }

    pub(crate) const fn 位置で渡す値を定義する(
        見出し: &'static str, 説明: &'static str, 扱い: 省略したときの扱い
    ) -> Self {
        Self::位置で渡す値 { 見出し, 説明, 扱い }
    }

    pub(crate) const fn 位置で何個でも渡す値を定義する(
        見出し: &'static str, 説明: &'static str, 扱い: 省略したときの扱い
    ) -> Self {
        Self::位置で何個でも渡す値 { 見出し, 説明, 扱い }
    }

    pub(crate) const fn そのまま子へ渡す残りの語を定義する(見出し: &'static str, 説明: &'static str) -> Self {
        Self::そのまま子へ渡す残りの語 { 見出し, 説明 }
    }

    /// 案内に出すこの引数の見出し。名前を持つ引数は綴りそのものが見出しになる。
    pub(crate) fn 見出し(self) -> &'static str {
        match self {
            Self::有無だけの旗 { 綴り, .. } | Self::名前に続けて渡す値 { 綴り, .. } => 綴り,
            Self::綴りから1つ選ぶ値 { 見出し, .. }
            | Self::位置で渡す値 { 見出し, .. }
            | Self::位置で何個でも渡す値 { 見出し, .. }
            | Self::そのまま子へ渡す残りの語 { 見出し, .. } => 見出し,
        }
    }

    /// 名前を持たず、コマンド行の位置だけで意味が決まる形か。1つ省くと後ろの位置がずれるため、
    /// フォームは省かれた後の位置の引数を尋ねない。
    pub(crate) fn 位置で意味が決まるか(self) -> bool {
        match self {
            Self::位置で渡す値 { .. } | Self::位置で何個でも渡す値 { .. } => true,
            Self::綴りから1つ選ぶ値 { 名前, .. } => 名前.is_none(),
            Self::有無だけの旗 { .. } | Self::名前に続けて渡す値 { .. } | Self::そのまま子へ渡す残りの語 { .. } => false,
        }
    }
}
