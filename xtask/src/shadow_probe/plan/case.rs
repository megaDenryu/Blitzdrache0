//! 1つの計測条件が持つものと、その組み立て。担当するのは「条件が何を既定から変えるか」を型で表すことである。
//!
//! 変えられるのは起動指定・アセットの世界・一日内時刻の3つであり、条件はそのうち1つだけを既定から動かす。
//! 3つを別々の組み立てとして名前で分けるのは、台帳の1行を読んだだけでその条件が何を振っているか分かるようにするためと、
//! 起動指定の綴りで世界や時刻を渡す形(同じ引数を2回渡して後勝ちに頼る形)を作らないためである。

use super::計測世界;

/// 条件が使う一日内時刻。太陽高度の軸だけが秒を固定し、他の軸は実行の指定(既定は17時)へ従う。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::shadow_probe) enum 条件の時刻 {
    実行の指定に従う,
    秒で固定(u32),
}

/// 1つの条件。名前は表と生値ファイルの見出しになり、起動指定はそのまま`blitz_app`へ渡る。
pub(in crate::shadow_probe) struct 計測条件 {
    pub(in crate::shadow_probe) 名前: &'static str,
    pub(in crate::shadow_probe) 起動指定: &'static [&'static str],
    pub(in crate::shadow_probe) 世界: 計測世界,
    pub(in crate::shadow_probe) 時刻: 条件の時刻,
}

impl 計測条件 {
    /// 起動指定だけを振る条件。世界も時刻も既定のままである。
    pub(in crate::shadow_probe) const fn 起動指定を振る(名前: &'static str, 起動指定: &'static [&'static str]) -> Self {
        Self {
            名前,
            起動指定,
            世界: 計測世界::代表の地形世界,
            時刻: 条件の時刻::実行の指定に従う,
        }
    }

    /// アセットの世界だけを振る条件。起動指定を空に保つことで、条件間の差が世界のアセットだけになる。
    pub(in crate::shadow_probe) const fn 世界を振る(名前: &'static str, 世界: 計測世界) -> Self {
        Self {
            名前,
            起動指定: &[],
            世界,
            時刻: 条件の時刻::実行の指定に従う,
        }
    }

    /// 一日内時刻だけを振る条件。
    pub(in crate::shadow_probe) const fn 時刻を振る(名前: &'static str, 秒: u32) -> Self {
        Self {
            名前,
            起動指定: &[],
            世界: 計測世界::代表の地形世界,
            時刻: 条件の時刻::秒で固定(秒),
        }
    }
}
