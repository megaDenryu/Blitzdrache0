//! 実行の指定の引数解釈。
//! 担当するのは引数の語から実行の指定を作ることだけであり、どの条件をどの順で起動するかは`schedule`が、1回ぶんの起動は`run`が、値の読み取りは`parse`が持つ。

pub(super) struct 実行の指定 {
    /// 1実行あたりに描くフレーム数。計器の窓は直近60フレームであり、既定はその倍以上を先に流して窓を整える。
    pub(super) フレーム数: u32,
    /// `--time-of-day`で与えられた一日内秒。指定が無ければ世界の方針の既定時刻で走る。
    pub(super) 一日内秒: Option<String>,
}

/// 既定のフレーム数。`sky-lut`が1条件を走らせる長さと同じであり、計器の60フレーム窓の2倍以上を先に流す。
const 既定のフレーム数: u32 = 160;

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の指定, String> {
    let mut フレーム数 = 既定のフレーム数;
    let mut 一日内秒 = None;
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        match 語.as_str() {
            "--frames" => フレーム数 = 数を読む("--frames", 残り.next())?,
            "--time-of-day" => 一日内秒 = Some(小数の語を読む("--time-of-day", 残り.next())?),
            _ => return Err(format!("知らない引数({語})。使えるのは--framesと--time-of-dayである")),
        }
    }
    if フレーム数 == 0 {
        return Err("--framesは1以上である必要がある".to_string());
    }
    Ok(実行の指定 {
        フレーム数, 一日内秒
    })
}

fn 数を読む(引数名: &str, 語: Option<&String>) -> Result<u32, String> {
    let 語 = 語.ok_or_else(|| format!("{引数名}の次に値が無い"))?;
    語.parse().map_err(|誤り| format!("{引数名}の値を数として読めない({語}): {誤り}"))
}

/// 一日内秒は子プロセスへそのまま渡すため語のまま持つ。ここで数として読むのは、綴り誤りを起動前に落とすためである。
fn 小数の語を読む(引数名: &str, 語: Option<&String>) -> Result<String, String> {
    let 語 = 語.ok_or_else(|| format!("{引数名}の次に値が無い"))?;
    語.parse::<f64>()
        .map_err(|誤り| format!("{引数名}の値を数として読めない({語}): {誤り}"))?;
    Ok(語.clone())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 既定はフレーム数160で時刻の指定が無い() {
        let 指定 = 引数を読む(&[]).unwrap();
        assert_eq!(指定.フレーム数, 既定のフレーム数);
        assert!(指定.一日内秒.is_none());
    }

    #[test]
    fn 知らない引数と零フレームは落とす() {
        assert!(引数を読む(&["--rounds".to_string()]).is_err());
        assert!(引数を読む(&["--frames".to_string(), "0".to_string()]).is_err());
        assert!(引数を読む(&["--time-of-day".to_string(), "昼".to_string()]).is_err());
    }

    #[test]
    fn 値の指定は語のまま持つ() {
        let 指定 = 引数を読む(&["--time-of-day".to_string(), "61200".to_string()]).unwrap();
        assert_eq!(指定.一日内秒.as_deref(), Some("61200"));
    }
}
