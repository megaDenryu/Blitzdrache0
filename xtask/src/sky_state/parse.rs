//! 報告の行を鍵と値の対へ写す工程。受け取るのはアプリの標準出力、返すのは代表時刻ごとの状態行の一覧である。
//! 行の綴りは`crates/blitz_app/src/reports/sky_state/row.rs`の出力と一致させている。

const 行の見出し: &str = "天空状態行 ";
const 系列の鍵: &str = "系列";
const 名前の鍵: &str = "名前";
pub(super) const 世界時刻の鍵: &str = "世界時刻秒";
pub(super) const 高度の鍵: &str = "太陽高度度";
pub(super) const 方向の鍵一覧: [&str; 3] = ["太陽方向東", "太陽方向天頂", "太陽方向南"];

pub(super) struct 状態行 {
    pub(super) 系列: String,
    pub(super) 名前: String,
    /// 系列と名前を除く鍵と値の対。値は報告が出した文字列のままであり、丸めも並べ替えもしない。
    pub(super) 項目: Vec<(String, String)>,
}

impl 状態行 {
    pub(super) fn 値(&self, 鍵: &str) -> Option<&str> {
        self.項目.iter().find(|(項目名, _)| 項目名 == 鍵).map(|(_, 値)| 値.as_str())
    }

    pub(super) fn 数値(&self, 鍵: &str) -> Result<f64, String> {
        let 値 = self
            .値(鍵)
            .ok_or_else(|| format!("系列{}・名前{}の行に「{鍵}」が無い", self.系列, self.名前))?;
        値.parse::<f64>()
            .map_err(|誤り| format!("系列{}・名前{}の「{鍵}」を数として読めない({値}): {誤り}", self.系列, self.名前))
    }
}

pub(super) fn 系列を取る<'行>(行一覧: &'行 [状態行], 名前: &str, 系列: &str) -> Result<&'行 状態行, String> {
    行一覧
        .iter()
        .find(|行| 行.名前 == 名前 && 行.系列 == 系列)
        .ok_or_else(|| format!("名前{名前}の系列{系列}の行が無い"))
}

pub(super) fn 行一覧を取り出す(標準出力: &str) -> Result<Vec<状態行>, String> {
    let mut 一覧 = Vec::new();
    for 行 in 標準出力.lines().filter(|行| 行.starts_with(行の見出し)) {
        一覧.push(一行を読む(行.trim_start_matches(行の見出し))?);
    }
    if 一覧.is_empty() {
        return Err("報告に天空状態行が1件も無い".to_string());
    }
    Ok(一覧)
}

fn 一行を読む(本文: &str) -> Result<状態行, String> {
    let mut 系列 = None;
    let mut 名前 = None;
    let mut 項目 = Vec::new();
    for 語 in 本文.split_whitespace() {
        let (鍵, 値) = 語.split_once('=').ok_or_else(|| format!("項{語}が鍵=値の形でない"))?;
        match 鍵 {
            系列の鍵 => 系列 = Some(値.to_string()),
            名前の鍵 => 名前 = Some(値.to_string()),
            _ => 項目.push((鍵.to_string(), 値.to_string())),
        }
    }
    Ok(状態行 {
        系列: 系列.ok_or_else(|| format!("行に系列が無い: {本文}"))?,
        名前: 名前.ok_or_else(|| format!("行に名前が無い: {本文}"))?,
        項目,
    })
}
