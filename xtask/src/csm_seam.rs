//! OW5第5段の検収入口。カスケードの帯の継ぎ目が絵に段差として出ないことを画素で確かめる。
//!
//! 長い受光面(検証用地形世界の5×5チャンク)と反復する遮蔽物(各チャンクの植生の群)を、低い太陽と寝かせた視線で描く。
//! この構図では4つの帯の境界がすべて画面を横切る。同じ条件を2回描き、1回は本番の色、もう1回は
//! 「どの帯を引いたか」と「影の中か」の可視化を出す。帯の領域は可視化から読み、輝度の段差は本番の絵から測る
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「検証の設計」)。
//!
//! 判定は3つである。どの帯にも影の画素があること、境界を挟む両側それぞれに影の画素があること、
//! 境界の輝度段差が同じ幅ぶん内側へ入った位置との差(帯の内側の勾配)を超えないことである。
//! 遷移帯の重みが単調で和1であることは`blitz_render`のユニットテストが持つ。

mod band_map;
mod boundary;
mod judgment;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/csm_seam";
/// 境界の数。帯数4に対して3つである(xtaskはエンジンのクレートへ依存しないため値を二重に持つ)。
const 境界の数: usize = 3;
/// 境界からの距離で、近傍とみなす画素の幅(画素)。狭すぎると標本が足りず、広すぎると境界から離れた陰影を混ぜる。
const 帯の幅: usize = 6;
/// 境界の近傍で片側に要る影の画素数の下限。これを下回ると「その側に影がある」と言えない。
const 影画素数の下限: usize = 100;
/// 輝度段差の絶対上限(8bit)。帯の内側の勾配が小さい構図でも、これを超える段差は目に見える継ぎ目である。
const 段差の絶対上限: f64 = 12.0;

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] csm-seam成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] csm-seam失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::地形世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 本番 = run::描画する(&出力先, "scene", false)?;
    let 可視化 = run::描画する(&出力先, "bands", true)?;
    if 本番.幅 != 可視化.幅 || 本番.高さ != 可視化.高さ {
        return Err("本番と可視化の読み戻し寸法が違う".to_string());
    }
    let 地図 = band_map::帯地図::読み取る(&可視化);

    let 帯別影画素数 = judgment::全帯に影があることを検査する(&地図)?;
    let 境界の影 = judgment::境界の両側に影があることを検査する(&地図)?;
    let 段差 = judgment::境界の輝度段差を検査する(&地図, &本番)?;

    let 本番png = crate::raw_png::変換する(&出力先.join("scene"))?;
    let 可視化png = crate::raw_png::変換する(&出力先.join("bands"))?;
    let 段差の記述: Vec<String> = 段差
        .iter()
        .enumerate()
        .map(|(番号, &(段差, 勾配))| format!("境界{番号}は段差{段差:.2}(帯の内側の勾配{勾配:.2})"))
        .collect();
    Ok(format!(
        "帯別の影画素数{帯別影画素数:?}、境界の両側の影画素数{境界の影:?}、{}、絵は{}と{}",
        段差の記述.join("・"),
        本番png.display(),
        可視化png.display()
    ))
}
