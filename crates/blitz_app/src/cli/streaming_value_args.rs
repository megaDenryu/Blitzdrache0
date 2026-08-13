//! 固定経路とチャンク読込器の値付き引数を、型付きの起動設定へ反映する。

use std::slice::Iter;

use blitz_engine::チャンク読込設定;

use super::streaming_settings::固定経路起動設定;
use super::{value_args::次の値を読む, 起動引数エラー};

#[cfg(test)]
mod tests;

pub(super) fn 固定経路引数を反映する(
    設定: &mut 固定経路起動設定,
    引数: &mut Iter<String>,
    引数名: &str,
) -> Result<(), 起動引数エラー> {
    let 値 = 次の値を読む(引数, 引数名, 起動引数エラー::固定経路不正)?;
    let メートル = 値
        .parse::<f64>()
        .map_err(|誤り| 起動引数エラー::固定経路不正(format!("{引数名}: {誤り}")))?;
    if !メートル.is_finite() {
        return Err(起動引数エラー::固定経路不正(format!("{引数名}は有限値でなければならない")));
    }
    if 引数名 == "--streaming-route-meters-per-frame" && メートル <= 0.0 {
        return Err(起動引数エラー::固定経路不正(format!("{引数名}は正でなければならない")));
    }
    match 引数名 {
        "--streaming-route-start-east-meters" => 設定.始点東メートル = メートル,
        "--streaming-route-start-south-meters" => 設定.始点南メートル = メートル,
        "--streaming-route-end-east-meters" => 設定.終点東メートル = メートル,
        "--streaming-route-end-south-meters" => 設定.終点南メートル = メートル,
        "--streaming-route-meters-per-frame" => 設定.一フレーム移動量メートル = メートル,
        _ => return Err(起動引数エラー::固定経路不正(format!("知らない引数である: {引数名}"))),
    }
    Ok(())
}

pub(super) fn 読込引数を反映する(
    現在: チャンク読込設定,
    引数: &mut Iter<String>,
    引数名: &str,
) -> Result<チャンク読込設定, 起動引数エラー> {
    let 値 = 次の値を読む(引数, 引数名, 起動引数エラー::チャンク読込設定不正)?;
    let 数 = 値
        .parse::<usize>()
        .map_err(|誤り| 起動引数エラー::チャンク読込設定不正(format!("{引数名}: {誤り}")))?;
    let (ワーカー, 要求, 完了) = match 引数名 {
        "--streaming-loader-workers" => (数, 現在.要求キュー容量(), 現在.完了キュー容量()),
        "--streaming-request-capacity" => (現在.ワーカー本数(), 数, 現在.完了キュー容量()),
        "--streaming-completion-capacity" => (現在.ワーカー本数(), 現在.要求キュー容量(), 数),
        _ => {
            return Err(起動引数エラー::チャンク読込設定不正(
                format!("知らない引数である: {引数名}"),
            ));
        }
    };
    チャンク読込設定::生成する(ワーカー, 要求, 完了).map_err(|誤り| 起動引数エラー::チャンク読込設定不正(誤り.to_string()))
}
