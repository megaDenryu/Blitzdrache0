//! XPBDの並列方式の計測の起動指定を見分けて読む工程。`--report-xpbd-solver-bench`が並びに在るときだけ、
//! 方式・グラフ・反復回数・刻み数・点の数を読んで指定を組み立てる。ウィンドウもシーンも読まない要求であるため、
//! 描画の設定の解析より前に見分ける。方式とグラフは既定を持たない(1条件が1プロセスであり、黙って既定を測る形を避ける)。

use blitz_render::xpbd_solver_bench_probe::XPBD並列方式;

use super::argument_error::起動引数エラー;

const 要求の引数名: &str = "--report-xpbd-solver-bench";
const 方式の引数名: &str = "--xpbd-method";
const グラフの引数名: &str = "--xpbd-graph";
const 反復回数の引数名: &str = "--xpbd-iterations";
const 刻み数の引数名: &str = "--xpbd-steps";
const 点の数の引数名: &str = "--xpbd-points";
const 比較の刻み数の引数名: &str = "--xpbd-compare-steps";
const 既定の反復回数: u32 = 4;
const 既定の刻み数: u32 = 240;
const 既定の点の数: u32 = 1024;
const 既定の比較の刻み数: u32 = 10;

/// 計測の題材にする拘束グラフの種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XPBD計測のグラフの種別 {
    規則格子,
    不規則,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct XPBD並列方式計測の指定 {
    pub(crate) 方式: XPBD並列方式,
    pub(crate) グラフ: XPBD計測のグラフの種別,
    pub(crate) 反復回数: u32,
    pub(crate) 刻み数: u32,
    pub(crate) 点の数: u32,
    pub(crate) 比較の刻み数: u32, // CPUの参照計算と突き合わせる短い実行の刻み数。長い実行では単精度の差が力学で増幅されて突き合わせにならない
}

/// 要求の引数が無ければ`None`。在れば残りの引数を読み、読めない語は型付きエラーにする。
pub(super) fn 計測の要求を見分ける(引数一覧: &[String]) -> Option<Result<XPBD並列方式計測の指定, 起動引数エラー>> {
    if !引数一覧.iter().any(|引数| 引数 == 要求の引数名) {
        return None;
    }
    Some(指定を読む(引数一覧))
}

fn 指定を読む(引数一覧: &[String]) -> Result<XPBD並列方式計測の指定, 起動引数エラー> {
    let mut 方式 = None;
    let mut グラフ = None;
    let mut 反復回数 = 既定の反復回数;
    let mut 刻み数 = 既定の刻み数;
    let mut 点の数 = 既定の点の数;
    let mut 比較の刻み数 = 既定の比較の刻み数;
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        match 語.as_str() {
            方式の引数名 => 方式 = Some(方式を読む(値を求める(方式の引数名, 残り.next())?)?),
            グラフの引数名 => グラフ = Some(グラフを読む(値を求める(グラフの引数名, 残り.next())?)?),
            反復回数の引数名 => 反復回数 = 数を読む(反復回数の引数名, 残り.next())?,
            刻み数の引数名 => 刻み数 = 数を読む(刻み数の引数名, 残り.next())?,
            点の数の引数名 => 点の数 = 数を読む(点の数の引数名, 残り.next())?,
            比較の刻み数の引数名 => 比較の刻み数 = 数を読む(比較の刻み数の引数名, 残り.next())?,
            _ => {}
        }
    }
    Ok(XPBD並列方式計測の指定 {
        方式: 方式.ok_or_else(|| 指定不正のエラーを組む(format!("{方式の引数名}が無い")))?,
        グラフ: グラフ.ok_or_else(|| 指定不正のエラーを組む(format!("{グラフの引数名}が無い")))?,
        反復回数,
        刻み数,
        点の数,
        比較の刻み数,
    })
}

fn 方式を読む(語: &str) -> Result<XPBD並列方式, 起動引数エラー> {
    match 語 {
        "atomic" => Ok(XPBD並列方式::原子加算),
        "coloring" => Ok(XPBD並列方式::グラフ彩色),
        "two-stage" => Ok(XPBD並列方式::二段階),
        _ => Err(指定不正のエラーを組む(format!(
            "{方式の引数名}はatomic・coloring・two-stageのどれかである({語})"
        ))),
    }
}

fn グラフを読む(語: &str) -> Result<XPBD計測のグラフの種別, 起動引数エラー> {
    match 語 {
        "grid" => Ok(XPBD計測のグラフの種別::規則格子),
        "irregular" => Ok(XPBD計測のグラフの種別::不規則),
        _ => Err(指定不正のエラーを組む(
            format!("{グラフの引数名}はgrid・irregularのどちらかである({語})"),
        )),
    }
}

fn 数を読む(引数名: &'static str, 語: Option<&String>) -> Result<u32, 起動引数エラー> {
    let 語 = 値を求める(引数名, 語)?;
    let 値: u32 = 語
        .parse()
        .map_err(|_| 指定不正のエラーを組む(format!("{引数名}の値を数として読めない({語})")))?;
    if 値 == 0 {
        return Err(指定不正のエラーを組む(format!("{引数名}は1以上である")));
    }
    Ok(値)
}

fn 値を求める<'語>(引数名: &'static str, 語: Option<&'語 String>) -> Result<&'語 str, 起動引数エラー> {
    語.map(String::as_str)
        .ok_or_else(|| 指定不正のエラーを組む(format!("{引数名}の次に値が無い")))
}

fn 指定不正のエラーを組む(理由: String) -> 起動引数エラー {
    起動引数エラー::XPBD並列方式計測の指定不正(理由)
}
