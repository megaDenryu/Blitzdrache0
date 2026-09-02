//! 実行の指定の引数解釈。担当するのは引数の語から実行の指定を作ることだけであり、どの条件をどの順で起動するかは
//! `schedule`が、1回ぶんの起動は`run`が持つ。方式とグラフは絞れるが、既定は全方式×全グラフである。

use super::error::XPBDの並列方式の計測エラー;
use super::schedule::{グラフ, 方式};

pub(super) struct 実行の指定 {
    pub(super) 方式一覧: Vec<方式>,
    pub(super) グラフ一覧: Vec<グラフ>,
    pub(super) 反復回数: u32,
    pub(super) 刻み数: u32,       // 1実行の刻み数。計器の窓は直近60刻みであり、既定はその4倍を流す
    pub(super) 点の数: u32,       // 規則格子では一辺の2乗でなければならない
    pub(super) 比較の刻み数: u32, // CPUの参照計算と突き合わせる短い実行の刻み数
}

const 方式の引数名: &str = "--method";
const グラフの引数名: &str = "--graph";
const 反復回数の引数名: &str = "--iterations";
const 刻み数の引数名: &str = "--steps";
const 点の数の引数名: &str = "--points";
const 比較の刻み数の引数名: &str = "--compare-steps";

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の指定, XPBDの並列方式の計測エラー> {
    let mut 指定 = 実行の指定 {
        方式一覧: 方式::全部().to_vec(),
        グラフ一覧: グラフ::全部().to_vec(),
        反復回数: 4,
        刻み数: 240,
        点の数: 1024,
        比較の刻み数: 10,
    };
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        match 語.as_str() {
            方式の引数名 => 指定.方式一覧 = 方式を読む(値の語を求める(方式の引数名, 残り.next())?)?,
            グラフの引数名 => 指定.グラフ一覧 = グラフを読む(値の語を求める(グラフの引数名, 残り.next())?)?,
            反復回数の引数名 => 指定.反復回数 = 数を読む(反復回数の引数名, 残り.next())?,
            刻み数の引数名 => 指定.刻み数 = 数を読む(刻み数の引数名, 残り.next())?,
            点の数の引数名 => 指定.点の数 = 数を読む(点の数の引数名, 残り.next())?,
            比較の刻み数の引数名 => 指定.比較の刻み数 = 数を読む(比較の刻み数の引数名, 残り.next())?,
            _ => return Err(XPBDの並列方式の計測エラー::知らない引数を渡された { 語: 語.clone() }),
        }
    }
    Ok(指定)
}

fn 方式を読む(語: &str) -> Result<Vec<方式>, XPBDの並列方式の計測エラー> {
    if 語 == "all" {
        return Ok(方式::全部().to_vec());
    }
    方式::全部()
        .iter()
        .find(|方式| 方式.起動指定の語() == 語)
        .map(|方式| vec![*方式])
        .ok_or_else(|| XPBDの並列方式の計測エラー::引数の値を読めない {
            引数名: 方式の引数名,
            語: 語.to_string(),
        })
}

fn グラフを読む(語: &str) -> Result<Vec<グラフ>, XPBDの並列方式の計測エラー> {
    if 語 == "both" {
        return Ok(グラフ::全部().to_vec());
    }
    グラフ::全部()
        .iter()
        .find(|グラフ| グラフ.起動指定の語() == 語)
        .map(|グラフ| vec![*グラフ])
        .ok_or_else(|| XPBDの並列方式の計測エラー::引数の値を読めない {
            引数名: グラフの引数名,
            語: 語.to_string(),
        })
}

fn 数を読む(引数名: &'static str, 語: Option<&String>) -> Result<u32, XPBDの並列方式の計測エラー> {
    let 語 = 値の語を求める(引数名, 語)?;
    let 値: u32 = 語
        .parse()
        .map_err(|_| XPBDの並列方式の計測エラー::引数の値を読めない { 引数名, 語: 語.clone() })?;
    if 値 == 0 {
        return Err(XPBDの並列方式の計測エラー::数が零である { 引数名 });
    }
    Ok(値)
}

fn 値の語を求める<'語>(引数名: &'static str, 語: Option<&'語 String>) -> Result<&'語 String, XPBDの並列方式の計測エラー> {
    語.ok_or(XPBDの並列方式の計測エラー::引数の次に値が無い { 引数名 })
}
