//! 周回数や物量といった実行の指定の引数解釈。担当するのは引数の語から実行の指定を作ることだけであり、
//! 軸そのものと綴りは`axis`が、軸ごとにどの条件を並べるかは`axis_cases`が、条件1つが何を既定から変えるかは`case`が、
//! 条件が使うアセットの世界は`measurement_world`が持つ。

mod axis;
mod axis_cases;
mod case;
mod measurement_world;
mod round_count;

pub(in crate::shadow_probe) use axis::振る軸;
pub(in crate::shadow_probe) use axis_cases::軸の条件一覧;
pub(in crate::shadow_probe) use case::{条件の時刻, 計測条件};
pub(in crate::shadow_probe) use measurement_world::計測世界;
pub(in crate::shadow_probe) use round_count::決める as 周回数を決める;

pub(super) struct 実行の指定 {
    pub(super) 軸: 振る軸,
    /// `--rounds`で与えられた周回数。実際に回す数は条件数が決まってから`周回数を決める`が求める。
    pub(super) 指定された周回数: Option<usize>,
    pub(super) チャンクあたり個体数: usize,
    pub(super) 一日内時刻の秒: u32,
}

/// 代表物量。25チャンクで常駐100,026個体になる密度であり、第8段が代表物量として選んだ点と同じである。
const 既定のチャンクあたり個体数: usize = 4_000;
/// 低い太陽(17時)。`csm-seam`が継ぎ目の検収に使う時刻と同じである。
const 既定の一日内時刻の秒: u32 = 61_200;

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の指定, String> {
    let mut 軸 = None;
    let mut 指定された周回数 = None;
    let mut チャンクあたり個体数 = 既定のチャンクあたり個体数;
    let mut 一日内時刻の秒 = 既定の一日内時刻の秒;
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        match 語.as_str() {
            "--rounds" => 指定された周回数 = Some(数を読む(語, 残り.next())?),
            "--chunk-instances" => チャンクあたり個体数 = 数を読む(語, 残り.next())?,
            "--time-of-day" => 一日内時刻の秒 = u32::try_from(数を読む(語, 残り.next())?).map_err(|誤り| format!("{誤り}"))?,
            _ => 軸 = Some(axis::綴りから読む(語)?),
        }
    }
    let 軸 = 軸.ok_or_else(|| format!("振る軸を1つ指定する({})", axis::綴りを並べる()))?;
    if 指定された周回数 == Some(0) {
        return Err("--roundsは1以上である必要がある".to_string());
    }
    Ok(実行の指定 {
        軸,
        指定された周回数,
        チャンクあたり個体数,
        一日内時刻の秒,
    })
}

fn 数を読む(引数名: &str, 語: Option<&String>) -> Result<usize, String> {
    let 語 = 語.ok_or_else(|| format!("{引数名}の次に値が無い"))?;
    語.parse().map_err(|誤り| format!("{引数名}の値を数として読めない({語}): {誤り}"))
}
