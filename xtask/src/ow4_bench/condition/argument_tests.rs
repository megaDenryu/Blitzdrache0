//! 計測条件の引数解釈を、既定と混在の並びで固定する検査。指定なしの実行が従来条件のままであることと、
//! 物量点・描画条件・時刻・律速切り分けの軸が同じ並びの中で取り違えられないことをここが押さえる。

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::super::*;

    fn 語列にする(語一覧: &[&str]) -> Vec<String> {
        語一覧.iter().map(|語| (*語).to_string()).collect()
    }

    /// 指定なしの実行が従来条件のままであること。ここが変わると過去の計測値と比べられなくなる。
    #[test]
    fn 既定は素の描画で時刻の指定を持たない() {
        let 読み = 引数を読む(&[]).unwrap();
        assert!(読み.物量点一覧.is_empty());
        assert_eq!(読み.条件.描画, 描画条件::素の描画);
        assert_eq!(読み.条件.一日内時刻の秒, None);
    }

    #[test]
    fn 物量点と条件を混ぜて並べられる() {
        let 読み = 引数を読む(&語列にする(&["4000", "--production-draw", "--time-of-day", "61200"])).unwrap();
        assert_eq!(読み.物量点一覧, vec![4000]);
        assert_eq!(読み.条件.描画, 描画条件::本番の描画);
        assert_eq!(読み.条件.一日内時刻の秒, Some(61200));
    }

    /// 律速切り分けの軸は指定が無ければ語を1つも足さない。ここが崩れると、指定なしの実行が過去の記録と比べられなくなる。
    #[test]
    fn 既定はシャドウの起動指定を1つも持たない() {
        assert!(引数を読む(&[]).unwrap().条件.シャドウ.起動指定().is_empty());
    }

    #[test]
    fn シャドウの軸を物量点や時刻と混ぜて並べられる() {
        let 語列 = 語列にする(&["4000", "--shadow-resolution", "1024", "--time-of-day", "61200", "--no-instance-shadow"]);
        let 読み = 引数を読む(&語列).unwrap();
        assert_eq!(読み.物量点一覧, vec![4000]);
        assert_eq!(読み.条件.一日内時刻の秒, Some(61200));
        assert_eq!(読み.条件.シャドウ.起動指定(), vec!["--shadow-resolution", "1024", "--no-instance-shadow"]);
    }

    #[test]
    fn 値の無い時刻指定と零の物量点は失敗にする() {
        assert!(引数を読む(&語列にする(&["--time-of-day"])).is_err());
        assert!(引数を読む(&語列にする(&["0"])).is_err());
    }
}
