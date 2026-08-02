//! 振る軸の選び方と、周回数や物量といった実行の指定の引数解釈。担当するのは引数の語から実行の指定を作ることだけであり、
//! 軸ごとにどの条件を並べるかは`axis_cases`が持つ。

mod axis_cases;

pub(in crate::shadow_probe) use axis_cases::軸の条件一覧;

/// 1つの条件。名前は表と生値ファイルの見出しになり、起動指定はそのまま`blitz_app`へ渡る。
pub(in crate::shadow_probe) struct 計測条件 {
    pub(in crate::shadow_probe) 名前: &'static str,
    pub(in crate::shadow_probe) 起動指定: &'static [&'static str],
}

/// 振る軸。1回の実行で1つの軸だけを振り、他は既定へ固定する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 振る軸 {
    解像度,
    キャスター,
    余白,
    視点,
}

pub(super) struct 実行の指定 {
    pub(super) 軸: 振る軸,
    pub(super) 周回数: usize,
    pub(super) チャンクあたり個体数: usize,
    pub(super) 一日内時刻の秒: u32,
}

/// 代表物量。25チャンクで常駐100,026個体になる密度であり、第8段が代表物量として選んだ点と同じである。
const 既定のチャンクあたり個体数: usize = 4_000;
/// 低い太陽(17時)。`csm-seam`が継ぎ目の検収に使う時刻と同じである。
const 既定の一日内時刻の秒: u32 = 61_200;
const 既定の周回数: usize = 4;

pub(super) fn 引数を読む(引数一覧: &[String]) -> Result<実行の指定, String> {
    let mut 軸 = None;
    let mut 周回数 = 既定の周回数;
    let mut チャンクあたり個体数 = 既定のチャンクあたり個体数;
    let mut 一日内時刻の秒 = 既定の一日内時刻の秒;
    let mut 残り = 引数一覧.iter();
    while let Some(語) = 残り.next() {
        match 語.as_str() {
            "--rounds" => 周回数 = 数を読む(語, 残り.next())?,
            "--chunk-instances" => チャンクあたり個体数 = 数を読む(語, 残り.next())?,
            "--time-of-day" => 一日内時刻の秒 = u32::try_from(数を読む(語, 残り.next())?).map_err(|誤り| format!("{誤り}"))?,
            _ => 軸 = Some(軸を読む(語)?),
        }
    }
    let 軸 = 軸.ok_or_else(|| "振る軸を1つ指定する(resolution / casters / margin / camera)".to_string())?;
    if 周回数 == 0 {
        return Err("--roundsは1以上である必要がある".to_string());
    }
    Ok(実行の指定 {
        軸,
        周回数,
        チャンクあたり個体数,
        一日内時刻の秒,
    })
}

fn 軸を読む(語: &str) -> Result<振る軸, String> {
    match 語 {
        "resolution" => Ok(振る軸::解像度),
        "casters" => Ok(振る軸::キャスター),
        "margin" => Ok(振る軸::余白),
        "camera" => Ok(振る軸::視点),
        _ => Err(format!(
            "知らない軸である({語})。resolution / casters / margin / camera のいずれかを指定する"
        )),
    }
}

fn 数を読む(引数名: &str, 語: Option<&String>) -> Result<usize, String> {
    let 語 = 語.ok_or_else(|| format!("{引数名}の次に値が無い"))?;
    語.parse().map_err(|誤り| format!("{引数名}の値を数として読めない({語}): {誤り}"))
}
