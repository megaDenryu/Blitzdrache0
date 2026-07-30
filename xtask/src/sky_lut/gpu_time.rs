//! パス別GPU時間の報告から、大気の4区間が別々に立っていることを確かめる工程。
//! 受け取るのは標準出力、返すのは4区間の名前とミリ秒を並べた文である。
//!
//! 4つを個別の区間にすることは設計正本が要求しており(「予算と計器」)、区間名はレンダーグラフのパス名そのままである。
//! 名前が1つでも欠けたら、そのパスが積まれていないか区間名が変わったことを意味するため失敗にする。

const 見出し: &str = "パス別GPU時間";
const 区間名一覧: [&str; 4] = ["透過率生成", "多重散乱生成", "スカイビュー生成", "空"];

pub(super) fn 四区間を読む(標準出力: &str) -> Result<String, String> {
    let 表: Vec<(&str, f64)> = 表の行を集める(標準出力)?;
    let mut 並び = Vec::new();
    for 区間名 in 区間名一覧 {
        let ミリ秒 = 表
            .iter()
            .find(|(名前, _)| *名前 == 区間名)
            .map(|(_, ミリ秒)| *ミリ秒)
            .ok_or_else(|| format!("{見出し}の表に区間{区間名}が無い"))?;
        並び.push(format!("{区間名}{ミリ秒:.4}ms"));
    }
    Ok(並び.join("・"))
}

/// 見出しの後に続く2桁の字下げの行を「名前: 値 ms」として読む。表の外の行は字下げで区切られる。
fn 表の行を集める(標準出力: &str) -> Result<Vec<(&str, f64)>, String> {
    let mut 行一覧 = 標準出力.lines().skip_while(|行| !行.contains(見出し));
    行一覧.next().ok_or_else(|| format!("標準出力に{見出し}の区画が無い"))?;
    let mut 表 = Vec::new();
    for 行 in 行一覧.take_while(|行| 行.starts_with("  ")) {
        let 中身 = 行.trim_start();
        let Some((名前, 残り)) = 中身.split_once(':') else {
            continue;
        };
        let 値 = 残り.split_whitespace().next().unwrap_or_default();
        let ミリ秒 = 値.parse().map_err(|誤り| format!("{名前}の値を数として読めない({行}): {誤り}"))?;
        表.push((名前, ミリ秒));
    }
    Ok(表)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const 標本: &str = "パス別GPU時間(移動平均、60フレーム窓):\n  透過率生成: 0.1000 ms\n  多重散乱生成: 0.2000 ms\n  スカイビュー生成: 0.3000 ms\n  空: 0.0154 ms\n";

    #[test]
    fn 四区間を並べる() {
        let 文 = 四区間を読む(標本).unwrap();
        assert!(文.contains("透過率生成0.1000ms"), "{文}");
        assert!(文.contains("空0.0154ms"), "{文}");
    }

    #[test]
    fn 区間が欠けたら失敗にする() {
        assert!(四区間を読む("パス別GPU時間:\n  空: 0.0154 ms\n").is_err());
    }
}
