//! 影の欠落計器の引数解釈。担当するのは、引数の語から構図の選択と候補の起動指定を作ることだけである。
//! 候補の起動指定をそのままの綴りで受けて渡すのは、この計器がαとβのどちらも同じ形で測るためであり、
//! 綴りの正本を`blitz_app`側に1つだけ置くためである。

use super::scene_choice;

pub(super) struct 指定 {
    pub(super) 構図: scene_choice::構図,
    /// 候補の実行だけへ足す起動指定。空なら基準と同じ絵になるため拒む。
    pub(super) 候補の起動指定: Vec<String>,
}

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<指定, String> {
    let mut 構図 = scene_choice::構図::地形;
    let mut 候補の起動指定 = Vec::new();
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        match 語.as_str() {
            "--layout" => {
                let 値 = 残り.next().ok_or_else(|| "--layoutの次に値が無い".to_string())?;
                構図 = scene_choice::綴りから読む(値)?;
            }
            "--max-shadow-distance" | "--shadow-caster-range" => {
                let 値 = 残り.next().ok_or_else(|| format!("{語}の次に値が無い"))?;
                候補の起動指定.push(語.clone());
                候補の起動指定.push(値.clone());
            }
            _ => return Err(format!("知らない引数である({語})")),
        }
    }
    if 候補の起動指定.is_empty() {
        return Err("候補の指定(--max-shadow-distanceまたは--shadow-caster-range)が要る".to_string());
    }
    Ok(指定 {
        構図, 候補の起動指定
    })
}
