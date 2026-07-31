//! 1回の実行へ渡す起動指定のうち、物量以外で変わりうるものを保つ型と、その引数の読み取りと起動指定への写し。
//! 物量(チャンクあたり個体数)を同じ型へ入れないのは、物量が物量点ごとに変わるのに対し、ここが持つ2つが
//! 全物量点で共有される条件だからである。既定を従来条件に置くのは、この計測が2026-07-29から続く時系列であり、
//! 指定なしの実行が過去の値と比べられる必要があるためである(参照: `_doc/計測/OW4_2026-07-29.md`)。

/// 何を描くかの条件。予算のどの行を判定できるかがこの選択で決まる。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 描画条件 {
    /// ライティングもポスト処理も空も止めた素の描画。シーン描画とシャドウの2行だけを過去の記録と比べられる。
    素の描画,
    /// 起動指定を1つも足さない本番の描画。空とポスト処理を含むため、空1.0msとGPU合計16.6msを判定できる。
    本番の描画,
}

pub(super) struct 計測条件 {
    pub(super) 描画: 描画条件,
    /// `--time-of-day <秒>`へ渡す一日内時刻。`None`なら世界の方針の既定時刻のままである。
    pub(super) 一日内時刻の秒: Option<u32>,
}

pub(super) struct 引数の読み {
    /// チャンクあたり個体数の列。空なら呼び出し元が既定の3点を使う。
    pub(super) 物量点一覧: Vec<usize>,
    pub(super) 条件: 計測条件,
}

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<引数の読み, String> {
    let mut 物量点一覧 = Vec::new();
    let mut 描画 = 描画条件::素の描画;
    let mut 一日内時刻の秒 = None;
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        match 語.as_str() {
            "--production-draw" => 描画 = 描画条件::本番の描画,
            "--time-of-day" => 一日内時刻の秒 = Some(一日内秒を読む(残り.next())?),
            _ => 物量点一覧.push(個体数を読む(語)?),
        }
    }
    Ok(引数の読み {
        物量点一覧,
        条件: 計測条件 {
            描画, 一日内時刻の秒
        },
    })
}

/// 描画条件を起動指定へ写す。地形世界は空を持つ方針であるため、従来条件を保つには空とライティングとポスト処理を明示的に外す。
pub(super) fn 描画の起動指定(描画: 描画条件) -> &'static [&'static str] {
    match 描画 {
        描画条件::素の描画 => &["--unlit", "--no-post", "--no-sky"],
        描画条件::本番の描画 => &[],
    }
}

fn 一日内秒を読む(語: Option<&String>) -> Result<u32, String> {
    let 語 = 語.ok_or_else(|| "--time-of-dayの次に秒が無い".to_string())?;
    語.parse().map_err(|誤り| format!("--time-of-dayの秒を数として読めない: {誤り}"))
}

fn 個体数を読む(語: &str) -> Result<usize, String> {
    match 語.parse::<usize>() {
        Ok(個体数) if 個体数 > 0 => Ok(個体数),
        Ok(_) => Err("チャンクあたり個体数は1以上である必要がある".to_string()),
        Err(誤り) => Err(format!("チャンクあたり個体数を数として読めない({語}): {誤り}")),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

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

    #[test]
    fn 値の無い時刻指定と零の物量点は失敗にする() {
        assert!(引数を読む(&語列にする(&["--time-of-day"])).is_err());
        assert!(引数を読む(&語列にする(&["0"])).is_err());
    }
}
