//! 建物定義が使う部品のソースを解決し、各部品の実体の境界箱を読む。

use std::path::PathBuf;

use blitz_assembly::{部品ID, 部品の境界箱};

use super::super::definition_source::建物定義の正本;
use super::super::error::建物外形カタログエラー;
use crate::loader::部品のglTFのファイル;
use crate::runtime_compilation::source_location::外部ソースルート;

pub(super) struct 部品の外形 {
    pub(super) 識別子: 部品ID,
    pub(super) 境界箱: 部品の境界箱,
}

pub(super) fn 部品のパス一覧を参照する(
    外部ルート: &外部ソースルート,
    正本: &建物定義の正本,
) -> Result<Vec<PathBuf>, 建物外形カタログエラー> {
    正本
        .規則
        .部品一覧()
        .iter()
        .map(|部品| {
            外部ルート.相対パスが指すソースを参照する(部品.ソース相対パス()).map_err(|原因| {
                建物外形カタログエラー::部品ソースを参照できない {
                    建物定義: 正本.識別子.to_string(),
                    原因,
                }
            })
        })
        .collect()
}

pub(super) fn 部品の外形一覧を読む(
    パス一覧: &[PathBuf],
    正本: &建物定義の正本,
) -> Result<Vec<部品の外形>, 建物外形カタログエラー> {
    パス一覧
        .iter()
        .map(|パス| {
            let ファイル = 部品のglTFのファイル::生成する(パス);
            match (ファイル.部品idを作る(), ファイル.境界箱を読み取る()) {
                (Ok(識別子), Ok(境界箱)) => Ok(部品の外形 { 識別子, 境界箱 }),
                (Err(原因), _) | (_, Err(原因)) => Err(建物外形カタログエラー::部品カタログを組み立てられない {
                    建物定義: 正本.識別子.to_string(),
                    原因: 原因.to_string(),
                }),
            }
        })
        .collect()
}
