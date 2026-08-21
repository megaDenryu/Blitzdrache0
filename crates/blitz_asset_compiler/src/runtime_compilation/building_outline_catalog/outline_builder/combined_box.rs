//! 展開済みの部品配置を走査し、建物全体の外接箱へ畳む。

use blitz_assembly::{群ローカルの箱, 部品ごとの配置表};

use super::super::definition_source::建物定義の正本;
use super::super::error::建物外形カタログエラー;
use super::part_outlines::部品の外形;

pub(super) fn 建物全体の箱を作る(
    正本: &建物定義の正本,
    部品一覧: &[部品の外形],
    配置表: &部品ごとの配置表,
) -> Result<群ローカルの箱, 建物外形カタログエラー> {
    let mut 全体: Option<群ローカルの箱> = None;
    for 据えた in 配置表.据えた順() {
        let 部品 = 部品一覧.iter().find(|部品| &部品.識別子 == 据えた.識別子()).ok_or_else(|| {
            建物外形カタログエラー::据えた部品の外形が無い {
                建物定義: 正本.識別子.to_string(),
                部品: 据えた.識別子().綴り().to_string(),
            }
        })?;
        let 箱 = 部品.境界箱.配置で写す(&据えた.配置());
        全体 = Some(match 全体 {
            Some(現在) => 現在.相手を含む箱(&箱),
            None => 箱,
        });
    }
    全体.ok_or_else(|| 建物外形カタログエラー::建物が空 {
        建物定義: 正本.識別子.to_string(),
    })
}
