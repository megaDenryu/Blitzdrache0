//! 跨ぎの報告の解析。受け取るのは報告の標準出力そのもの、返すのは跨ぎの一覧か、読めなかった理由である。
//!
//! 起動から分けるのは、この工程が文字列だけを材料にし、GPUもプロセスも要らないためである。単体検査が
//! 壊れた行と総数の食い違いを直に与えられる。
//!
//! どの検査も「読めた行だけを使う」ことをしない。1行の形式が変わっただけで検収が黙って少ない件数を測り、
//! 成功として終わってしまうためである。

/// 1つの跨ぎ。段差を測る対はこの跨ぎの下側と上側の区間で撮られる。
#[derive(Debug)]
pub(in super::super) struct 跨ぎ {
    pub(in super::super) 上側の区間識別: u16,
    /// 跨ぐ向きの語。報告が出すASCIIの語をそのまま持ち、ファイル名にも使う。
    pub(in super::super) 方向: String,
    pub(in super::super) 境界の天頂余弦: f64,
    pub(in super::super) 一日内秒: f64,
}

const 見出しの前置き: &str = "太陽天頂区間の跨ぎ報告 ";
const 行の前置き: &str = "太陽天頂区間の跨ぎ 番号=";

pub(in super::super) fn 一覧を読む(標準出力: &str) -> Result<Vec<跨ぎ>, String> {
    let 総数 = 見出しの総数を読む(標準出力)?;
    let mut 一覧 = Vec::with_capacity(総数);
    for 行 in 標準出力.lines().filter(|行| 行.starts_with(行の前置き)) {
        let (番号, 跨ぎ) = 行を読む(行)?;
        if 番号 != 一覧.len() {
            return Err(format!("跨ぎの番号が0からの連番でない: {}件目の行が番号{番号}である", 一覧.len()));
        }
        一覧.push(跨ぎ);
    }
    if 一覧.len() != 総数 {
        return Err(format!("跨ぎの行が{}件しか読めなかった(見出しは{総数}件と言っている)", 一覧.len()));
    }
    Ok(一覧)
}

fn 見出しの総数を読む(標準出力: &str) -> Result<usize, String> {
    let 見出し = 標準出力
        .lines()
        .find(|行| 行.starts_with(見出しの前置き))
        .ok_or_else(|| "跨ぎの報告に見出しの行が無い".to_string())?;
    値を取る(見出し, "跨ぎの総数=")?
        .parse()
        .map_err(|誤り| format!("跨ぎの総数を数として読めない: {誤り}"))
}

/// 跨ぎの行から番号と4つの値を読む。行の形は
/// `太陽天頂区間の跨ぎ 番号=.. 上側の区間識別=.. 方向=.. 境界の天頂余弦=.. ... 一日内秒=..`である。
fn 行を読む(行: &str) -> Result<(usize, 跨ぎ), String> {
    let 番号 = 数を読む(行, "番号=")?;
    Ok((
        番号,
        跨ぎ {
            上側の区間識別: 数を読む(行, "上側の区間識別=")?,
            方向: 値を取る(行, "方向=")?.to_string(),
            境界の天頂余弦: 数を読む(行, "境界の天頂余弦=")?,
            一日内秒: 数を読む(行, "一日内秒=")?,
        },
    ))
}

fn 数を読む<値: std::str::FromStr>(行: &str, 前置き: &str) -> Result<値, String>
where
    値::Err: std::fmt::Display,
{
    値を取る(行, 前置き)?
        .parse()
        .map_err(|誤り| format!("跨ぎの行の「{前置き}」を数として読めない: {行}({誤り})"))
}

fn 値を取る<'行>(行: &'行 str, 前置き: &str) -> Result<&'行 str, String> {
    行.split_once(前置き)
        .ok_or_else(|| format!("跨ぎの行に「{前置き}」が無い: {行}"))?
        .1
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("跨ぎの行の「{前置き}」に値が無い: {行}"))
}
