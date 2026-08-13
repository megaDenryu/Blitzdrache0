//! 計測引数の生の値を、正値・有限値・大規模世界のシーンという制約へ通す。

pub(super) fn 正の数を読む<T: std::str::FromStr + Default + PartialOrd>(名前: &str, 値: &str) -> Result<T, String> {
    let 数 = 値.parse::<T>().map_err(|_| 数の誤り(名前, 値))?;
    (数 > T::default())
        .then_some(数)
        .ok_or_else(|| format!("{名前}は1以上でなければならない"))
}

pub(super) fn 有限値を読む(名前: &str, 値: &str) -> Result<f64, String> {
    let 数 = 値.parse::<f64>().map_err(|_| 数の誤り(名前, 値))?;
    数.is_finite().then_some(数).ok_or_else(|| format!("{名前}は有限値でなければならない"))
}

pub(super) fn 正の有限値を読む(名前: &str, 値: &str) -> Result<f64, String> {
    let 数 = 有限値を読む(名前, 値)?;
    (数 > 0.0).then_some(数).ok_or_else(|| format!("{名前}は正でなければならない"))
}

pub(super) fn シーンを読む(値: &str) -> Result<String, String> {
    (値 == "terrain_fox_tour")
        .then(|| 値.to_string())
        .ok_or_else(|| format!("大規模世界で選べないシーンである: {値}"))
}

pub(super) fn 数の誤り(名前: &str, 値: &str) -> String {
    format!("{名前}の値を数として読めない({値})")
}
