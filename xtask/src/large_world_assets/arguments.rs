//! 大規模世界の東西・南北チャンク数を読む。省略時は最終目標の80×100とする。

use blitz_asset_compiler::世界の広がり;

const 東西の選択肢: &str = "--east-chunks";
const 南北の選択肢: &str = "--south-chunks";

pub(super) fn 世界の広がりを読む(引数一覧: &[String]) -> Result<世界の広がり, String> {
    if 引数一覧.is_empty() {
        return Ok(世界の広がり::大規模世界の既定値());
    }
    let mut 東西 = None;
    let mut 南北 = None;
    let mut 残り = 引数一覧;
    while let [選択肢, 値, 続き @ ..] = 残り {
        let 数 = 値.parse::<u16>().map_err(|誤り| format!("{選択肢}の値を読めない({値}): {誤り}"))?;
        match 選択肢.as_str() {
            東西の選択肢 => 東西 = Some(数),
            南北の選択肢 => 南北 = Some(数),
            _ => return Err(format!("知らない選択肢である: {選択肢}")),
        }
        残り = 続き;
    }
    if !残り.is_empty() {
        return Err(format!("{}に続く値が無い", 残り.join(" ")));
    }
    match (東西, 南北) {
        (Some(東西), Some(南北)) => 世界の広がり::生成する(東西, 南北).map_err(|誤り| 誤り.to_string()),
        _ => Err(format!("{東西の選択肢}と{南北の選択肢}は一緒に指定する")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 省略時は八十掛ける百である() {
        let Ok(広がり) = 世界の広がりを読む(&[]) else {
            panic!("既定値を読めなかった");
        };
        assert_eq!((広がり.東西チャンク数(), 広がり.南北チャンク数()), (80, 100));
    }

    #[test]
    fn 二十掛ける二十を指定できる() {
        let 引数 = vec![東西の選択肢.to_string(), "20".to_string(), 南北の選択肢.to_string(), "20".to_string()];
        let Ok(広がり) = 世界の広がりを読む(&引数) else {
            panic!("2km中間世界の指定を読めなかった");
        };
        assert_eq!((広がり.東西チャンク数(), 広がり.南北チャンク数()), (20, 20));
    }

    #[test]
    fn 片方だけの指定を拒む() {
        let 引数 = vec![東西の選択肢.to_string(), "20".to_string()];
        assert!(世界の広がりを読む(&引数).is_err());
    }
}
