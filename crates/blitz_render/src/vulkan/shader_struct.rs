//! slangの構造体の宣言から、各フィールドの開始位置と全体のバイト長を読み取る工程。
//! 受け取るのはslangの原文と構造体名、返すのはフィールド名と開始位置の並びとバイト長である。
//! シェーダー定数・個体レコード・材質レコード・描画定数の各検査がこの1つの読み取りを共有する。
//!
//! 扱うのはCPU側のバイト詰めと同じ規則、すなわち宣言の順に前から詰めた位置がそのまま開始位置になる並びに限る。
//! 4x4行列は64バイト、3x3行列は列ごとに16バイト境界へ載るため48バイト、vec4とuvec4は16バイトを占める。
//! float3の直後に4バイトの値を置く形は、float3が16バイト境界から始まりuintが4バイト境界に載るため詰めた位置と一致する。
//! 宣言に他の型が現れたら読み取り失敗として返し、「一致した」と読み替えない。読み取れなかった形の数え上げは`read_error`が持つ。

mod read_error;

pub(crate) use read_error::シェーダー構造体の読み取りの破れ;

pub(crate) struct シェーダー構造体の並び {
    pub(crate) フィールド一覧: Vec<(String, usize)>,
    pub(crate) バイト長: usize,
}

impl シェーダー構造体の並び {
    pub(crate) fn 開始位置(&self, 名前: &str) -> Option<usize> {
        self.フィールド一覧.iter().find(|(項目, _)| 項目 == 名前).map(|(_, 位置)| *位置)
    }
}

pub(crate) fn 読み取る(
    原文: &str,
    構造体名: &str,
) -> Result<シェーダー構造体の並び, シェーダー構造体の読み取りの破れ> {
    let 宣言 = format!("struct {構造体名}");
    let mut 行一覧 = 原文.lines().skip_while(|行| 行.trim() != 宣言);
    if 行一覧.next().is_none() {
        return Err(シェーダー構造体の読み取りの破れ::宣言が見つからない {
            構造体名: 構造体名.to_string(),
        });
    }
    let mut フィールド一覧 = Vec::new();
    let mut 位置 = 0usize;
    for 行 in 行一覧 {
        let 内容 = 行.trim();
        if 内容 == "{" || 内容.is_empty() || 内容.starts_with("//") {
            continue;
        }
        if 内容 == "};" {
            return Ok(シェーダー構造体の並び {
                フィールド一覧,
                バイト長: 位置,
            });
        }
        let (名前, 大きさ) = フィールドを読む(内容)?;
        フィールド一覧.push((名前, 位置));
        位置 += 大きさ;
    }
    Err(シェーダー構造体の読み取りの破れ::宣言が閉じていない {
        構造体名: 構造体名.to_string(),
    })
}

/// 「[column_major] 型 名前[個数];」の1行から、フィールド名と占めるバイト数を求める。
fn フィールドを読む(行: &str) -> Result<(String, usize), シェーダー構造体の読み取りの破れ> {
    let 語一覧: Vec<&str> = 行
        .trim_end_matches(';')
        .split_whitespace()
        .skip_while(|語| *語 == "column_major")
        .collect();
    let [型, 宣言子] = 語一覧.as_slice() else {
        return Err(シェーダー構造体の読み取りの破れ::フィールドの宣言として読めない { 行: 行.to_string() });
    };
    let 単位バイト数 = match *型 {
        "float4x4" => 64,
        "float3x3" => 48,
        "float4" => 16,
        "uint4" => 16,
        "float3" => 12,
        "float" => 4,
        "uint" => 4,
        その他 => {
            return Err(シェーダー構造体の読み取りの破れ::バイト詰めの規則が決まっていない型である {
                型名: その他.to_string(),
            });
        }
    };
    let Some((名前, 添字部)) = 宣言子.split_once('[') else {
        return Ok(((*宣言子).to_string(), 単位バイト数));
    };
    let 要素数の綴り = 添字部.trim_end_matches(']');
    let 個数: usize =
        要素数の綴り
            .parse()
            .map_err(|誤り| シェーダー構造体の読み取りの破れ::配列の要素数を数として読めない {
                綴り: 要素数の綴り.to_string(),
                誤り,
            })?;
    Ok((名前.to_string(), 単位バイト数 * 個数))
}
