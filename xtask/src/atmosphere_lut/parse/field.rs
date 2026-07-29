//! 報告の1行から鍵つきの値を取り出す工程。受け取るのは空白で切った語の一覧と鍵、返すのは整数または実数である。
//! 語の並びに依存せず鍵で引くのは、報告へ項目を足したときに読み取り側が壊れないようにするためである。

fn 値を探す<'a>(語一覧: &[&'a str], 鍵: &str) -> Result<&'a str, String> {
    let 接頭辞 = format!("{鍵}=");
    語一覧
        .iter()
        .find_map(|語| 語.strip_prefix(&接頭辞))
        .ok_or_else(|| format!("報告の行に{鍵}が無い"))
}

pub(super) fn 整数値(語一覧: &[&str], 鍵: &str) -> Result<usize, String> {
    値を探す(語一覧, 鍵)?
        .parse::<usize>()
        .map_err(|誤り| format!("{鍵}を整数として読めない: {誤り}"))
}

pub(super) fn 実数値(語一覧: &[&str], 鍵: &str) -> Result<f64, String> {
    値を探す(語一覧, 鍵)?
        .parse::<f64>()
        .map_err(|誤り| format!("{鍵}を実数として読めない: {誤り}"))
}
