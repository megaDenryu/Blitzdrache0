//! 影の欠落計器の引数解釈。担当するのは、引数の語から構図の選択とちょうど1つの候補の指定を作ることである。
//!
//! 候補をちょうど1つだけ受けるのは、設計の正本が「αとβを同時に変えない」と定めるためである。
//! 1つも渡らない実行も、2つ渡った実行も、同じ軸を2回渡した実行もここで失敗にする。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

use super::candidate_axis::{候補の計測指定, 計測軸};
use super::scene_choice;

pub(super) struct 指定 {
    pub(super) 構図: scene_choice::構図,
    pub(super) 候補: 候補の計測指定,
}

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<指定, String> {
    let mut 構図: Option<scene_choice::構図> = None;
    let mut 候補: Option<候補の計測指定> = None;
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        if 語 == "--layout" {
            let 値 = 残り.next().ok_or_else(|| "--layoutの次に値が無い".to_string())?;
            if 構図.is_some() {
                return Err("--layoutが2回ある。1回の実行で描く構図は1つだけである".to_string());
            }
            構図 = Some(scene_choice::綴りから読む(値)?);
            continue;
        }
        let 軸 = 計測軸::綴りから読む(語).ok_or_else(|| format!("知らない引数である({語})"))?;
        let 値 = 残り.next().ok_or_else(|| format!("{語}の次に値が無い"))?;
        if let Some(既に読んだ候補) = &候補 {
            return Err(format!(
                "候補の指定が2つある({}と{})。同時に変えると、どちらの軸が絵を動かしたか分けられない",
                既に読んだ候補.綴り(),
                軸.綴り()
            ));
        }
        候補 = Some(軸.値を添える(値)?);
    }
    組み立てる(構図.unwrap_or(scene_choice::構図::地形), 候補)
}

fn 組み立てる(構図: scene_choice::構図, 候補: Option<候補の計測指定>) -> Result<指定, String> {
    let 候補 = 候補.ok_or_else(|| {
        format!(
            "候補の指定が1つ要る({})。ちょうど1つだけ受けるのは、αとβを同時に変えないためである",
            計測軸::綴りを並べる(&計測軸::全軸)
        )
    })?;
    let 受け入れる軸 = 構図.受け入れる軸();
    if !受け入れる軸.contains(&候補.軸()) {
        return Err(format!(
            "構図{}は{}の候補を受けない。この構図が受けるのは{}である",
            構図.綴り(),
            候補.綴り(),
            計測軸::綴りを並べる(受け入れる軸)
        ));
    }
    Ok(指定 { 構図, 候補 })
}

#[cfg(test)]
mod exclusivity_tests;
