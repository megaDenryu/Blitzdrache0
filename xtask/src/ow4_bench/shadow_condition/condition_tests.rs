//! 律速切り分けの軸の読み取りと起動指定への写しを、境界の値で固定する検査。
//! 指定なしの実行が語を1つも足さないことをここが押さえる。ここが崩れると過去の記録と比べられなくなる。

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::super::シャドウ計測指定;

    #[test]
    fn 指定が無ければ語を1つも足さない() {
        assert!(シャドウ計測指定::default().起動指定().is_empty());
    }

    #[test]
    fn 担当外の語はfalseを返して呼び出し元へ戻す() {
        let 語一覧: Vec<String> = Vec::new();
        let mut 指定 = シャドウ計測指定::default();
        assert!(!指定.語を読む("4000", &mut 語一覧.iter()).unwrap());
    }

    #[test]
    fn 値つきと旗を読んで起動指定へ写す() {
        let 語一覧: Vec<String> = ["4096".to_string()].to_vec();
        let mut 残り = 語一覧.iter();
        let mut 指定 = シャドウ計測指定::default();
        assert!(指定.語を読む("--shadow-resolution", &mut 残り).unwrap());
        assert!(指定.語を読む("--no-instance-shadow", &mut 残り).unwrap());
        assert_eq!(指定.起動指定(), vec!["--shadow-resolution", "4096", "--no-instance-shadow"]);
    }

    #[test]
    fn 値の無い指定は失敗にする() {
        let 語一覧: Vec<String> = Vec::new();
        assert!(シャドウ計測指定::default().語を読む("--caster-margin", &mut 語一覧.iter()).is_err());
    }
}
