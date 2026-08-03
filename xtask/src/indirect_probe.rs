//! 順3-Ic-2の検収入口。金属と誘電体の板を粗さの水準ごとに並べた専用の世界を本番のシーン描画経路で描き、
//! 遠方環境の派生表現へ注入した解析入力から組んだ期待値と読み戻した画素が一致することを判定する。
//!
//! 条件を3つに分けるのは、消費式の欠陥の所在を分離するためである。拡散だけを立てれば取り分と円周率の扱いが、
//! 鏡面の段ごとに違う色を置けば粗さから段への写像が、面ごとに違う色を置けば太陽相対座標への写しが独立に落ちる。
//! 焼いた空を唯一のオラクルにすると、この3つが混ざった1つの数しか出ない。
//!
//! 面の判定を2つの時刻で走らせるのは、1つの視点から参照できる反射方向が視線の周り120度以内に限られるためである
//! (法線と視線の余弦が0.5以上の板で反射方向が届く範囲)。太陽相対座標のフレームは太陽の方位で回るため、
//! 時刻を変えると同じ板の反射方向が別の面へ写り、2つの時刻の合併で6面すべてを覆える。
//!
//! 破綻防止帯だけは注入を外して大気から焼いた遠方環境で測る。注入した条件は判定のために金属を黒へ落とすなど
//! 極端な値を作るため、絵が壊れていないかを見る対象にならない。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Icの消費式と実装段割り」

mod band;
mod conditions;
mod face_judgment;
mod judgment;
mod parse;
mod run;
mod summary;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/indirect_probe";

/// 夜の一日内秒。太陽高度が地平線より十分下にあり、方向光の強度が0になる時刻である。間接光だけが板の画素に出る。
const 夜の一日内秒: &str = "0";
/// もう1つの夜の一日内秒。太陽の方位が真夜中と大きく違うため、太陽相対座標のフレームが回り、
/// 同じ板の反射方向が別の面を指す。
const 方位の違う夜の一日内秒: &str = "68277";
/// 破綻防止帯を測る時刻。太陽が高く、大気から焼いた遠方環境が最も明るくなる条件である。
const 昼の一日内秒: &str = "43200";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] indirect-probe成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] indirect-probe失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 拡散 = conditions::条件を判定する(&出力先, "diffuse", "diffuse", 夜の一日内秒)?;
    let 段別 = conditions::条件を判定する(&出力先, "specular_level", "specular-level", 夜の一日内秒)?;
    let 面別 = conditions::条件を判定する(&出力先, "specular_face", "specular-face", 夜の一日内秒)?;
    let 面別の別方位 = conditions::条件を判定する(&出力先, "specular_face_azimuth", "specular-face", 方位の違う夜の一日内秒)?;

    let 黒に落ちた金属 = judgment::金属が黒に落ちることを検査する(&拡散.行一覧, &段別.行一覧)?;
    judgment::段が整数であることを検査する(&段別.行一覧)?;
    let 面の最小余裕 = face_judgment::面の余裕を検査する(&面別.行一覧)?.min(face_judgment::面の余裕を検査する(&面別の別方位.行一覧)?);
    let 覆った面 = face_judgment::立方体の6面を覆うことを検査する(&[&面別.行一覧, &面別の別方位.行一覧])?;

    let 帯 = conditions::破綻防止帯を測る(&出力先, 昼の一日内秒)?;
    Ok(summary::要約を組む(&summary::要約の材料 {
        拡散: &拡散,
        段別: &段別,
        面別: &面別,
        面別の別方位: &面別の別方位,
        黒に落ちた金属,
        面の最小余裕,
        覆った面: &覆った面,
        帯: &帯,
    }))
}
