//! 判定の名前: 検収が課す1つの判定を指す名前。破れの文面で「何が破れたか」を名指す。
//!
//! 綴りを所有する形にするのは、パス別や区間別の判定が名前を実行時に組み立てるためである。
//! 原文のリテラルは借りたまま持てるため、固定の名前に確保の費用は掛からない。
//!
//! 破れの組み立てをこの型の口だけにするのは、名前の付かない破れを作れないようにするためである。
//! この名前が課す比べのうち、数直線上の位置は`order_imposition`が、等しさは`equality_imposition`が持つ。

mod equality_imposition;
mod order_imposition;

use std::borrow::Cow;

use super::error::{判定の破れ, 判定の破れの種類};
use super::judgment_value::判定が比べる値;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 判定の名前(Cow<'static, str>);

impl 判定の名前 {
    /// 原文のリテラルから作る。名前が空であることは書き誤りであるため、定数として使う限りコンパイル時に落ちる。
    pub const fn 定数から生成する(綴り: &'static str) -> Self {
        assert!(!綴り.is_empty(), "判定の名前が空である");
        Self(Cow::Borrowed(綴り))
    }

    /// 実行時に組み立てた綴りから作る。パス名や区間名を含む判定がこの口を通る。
    ///
    /// 注意: 名前の無い判定は破れの文面が何も名指せないため、空の綴りは到達したらバグとして落とす。
    pub fn 組み立てた綴りから生成する(綴り: String) -> Self {
        assert!(!綴り.is_empty(), "判定の名前が空である");
        Self(Cow::Owned(綴り))
    }

    /// 名前と種類を1つの破れへ組む。比べで表せない破れ(綴りの食い違い等)を課す側がこの口を通る。
    pub fn 破れを組む(&self, 種類: 判定の破れの種類) -> 判定の破れ {
        判定の破れ::名前と種類から組み立てる(self.clone(), 種類)
    }

    /// 数え上げの結果が空だったことを破れにする。件数0で返すと、続く統計が意味を持たないまま消費側へ渡る。
    pub fn あるはずのものが無い破れ(&self) -> 判定の破れ {
        self.破れを組む(判定の破れの種類::あるはずのものが1件も無い)
    }

    /// 判定の材料に置いた綴りを数として読み、読めなければ破れにする。報告が綴りのまま運ぶ値がこの口を通る。
    pub fn 材料を数として読む<数: std::str::FromStr>(&self, 材料: &str) -> Result<数, 判定の破れ> {
        材料.parse().map_err(|_| {
            self.破れを組む(判定の破れの種類::材料を数として読めない {
                材料: 判定が比べる値::綴り(材料.to_string()),
            })
        })
    }
}

impl std::fmt::Display for 判定の名前 {
    fn fmt(&self, 書き手: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(書き手, "{}", self.0)
    }
}
