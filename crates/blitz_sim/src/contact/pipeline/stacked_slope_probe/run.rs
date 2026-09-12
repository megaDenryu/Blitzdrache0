//! 指定の適用規則1つを通して坂の上の2段の動的な箱を600刻み走らせ、結末の内訳と破れの群を綴る工程。
//! 親モジュール(`stacked_slope_probe`)から分けているのは、こちらが1回の走行と読み取りの手順であり、
//! あちらが新形と旧の形の2回を対にして比べる検査だからである。

use std::collections::BTreeMap;

use super::super::acceptance_breach_reading::受理の破れの読み取り;
use super::super::stacked_slope_fixture::坂の上の二段の箱の場面;
use super::super::stacked_slope_spec::原点;
use crate::contact::friction_coefficient::摩擦係数;
use crate::contact::normal_tangential_system::{混合連立方程式の受理の結果, 試験の許容差の適用規則};

const 綴る刻みの数: usize = 600;
const 内訳を綴る刻みの数: usize = 3;
const 群ごとに綴る例の数: usize = 6;

fn 結末の鍵(結果: &混合連立方程式の受理の結果) -> &'static str {
    match 結果 {
        混合連立方程式の受理の結果::受理した => "受理",
        混合連立方程式の受理の結果::円錐を超えた => "円錐",
        混合連立方程式の受理の結果::数値の契約を破った(理由) => 理由.契約の名前(),
    }
}

fn 下の箱の坂の面に対する傾きの正接(場面: &坂の上の二段の箱の場面) -> f32 {
    let Ok(剛体) = 場面.台帳.参照する(場面.下の箱) else {
        panic!("箱が台帳に無い");
    };
    場面.幾何.坂の面に対する箱の傾きの正接(剛体.配置().姿勢())
}

fn 下り向きの変位(場面: &坂の上の二段の箱の場面) -> (f32, f32) {
    let 読む = |箱| {
        let Ok(剛体) = 場面.台帳.参照する(箱) else {
            panic!("箱が台帳に無い");
        };
        (剛体.配置().重心の位置() - 原点()).方向に沿う長さ(場面.幾何.下り向き()).値()
    };
    (読む(場面.下の箱), 読む(場面.上の箱))
}

pub(in crate::contact::pipeline) fn 一つの適用規則を綴る(見出し: &str, 適用規則: 試験の許容差の適用規則) {
    let Ok(摩擦) = 摩擦係数::生成する(0.6) else {
        panic!("摩擦係数を作れない");
    };
    let mut 場面 = 坂の上の二段の箱の場面::生成する(0.55, 摩擦);
    場面.工程.解法.計器.許容差の適用規則を差し替える(適用規則);
    let 初め = 下り向きの変位(&場面);
    let mut 全体: BTreeMap<&'static str, usize> = BTreeMap::new();
    let (mut 小さい群, mut 大きい群): (Vec<受理の破れの読み取り>, Vec<受理の破れの読み取り>) = (Vec::new(), Vec::new());
    let mut 下の箱の傾きの正接の最大 = 0.0_f32;
    println!("[{見出し}]");
    for 刻み in 0..綴る刻みの数 {
        let Ok(_) = 場面.工程.一刻み進める(&mut 場面.台帳) else {
            panic!("刻み {刻み} で一刻み進めるエラー");
        };
        下の箱の傾きの正接の最大 = 下の箱の傾きの正接の最大.max(下の箱の坂の面に対する傾きの正接(&場面).abs());
        let 履歴 = 場面.工程.解法.計器.受理の判定の履歴を読んで空にする();
        let mut 内訳: BTreeMap<&'static str, usize> = BTreeMap::new();
        for (結果, 読み取り) in &履歴 {
            *内訳.entry(結末の鍵(結果)).or_insert(0) += 1;
            *全体.entry(結末の鍵(結果)).or_insert(0) += 1;
            if let Some(読み取り) = 読み取り {
                if 読み取り.比が大きい群か() {
                    大きい群.push(読み取り.clone())
                } else {
                    小さい群.push(読み取り.clone())
                }
            }
        }
        if 刻み < 内訳を綴る刻みの数 || 刻み % 100 == 99 {
            let いま = 下り向きの変位(&場面);
            let 内訳の綴り: Vec<String> = 内訳.iter().map(|(鍵, 数)| format!("{鍵}={数}")).collect();
            println!(
                "  刻み{刻み} 判定{}件[{}] 下の箱の変位={:.3e} 上の箱の変位={:.3e}",
                履歴.len(),
                内訳の綴り.join(" "),
                いま.0 - 初め.0,
                いま.1 - 初め.1
            );
        }
    }
    let 全体の綴り: Vec<String> = 全体.iter().map(|(鍵, 数)| format!("{鍵}={数}")).collect();
    println!("  下の箱の坂の面に対する傾きの正接の最大={下の箱の傾きの正接の最大:.3e}");
    println!(
        "  600刻みの結末の延べ数[{}] B1の破れ 比<10 {}件 比>=10 {}件",
        全体の綴り.join(" "),
        小さい群.len(),
        大きい群.len()
    );
    for (名前, 群) in [("比<10", &小さい群), ("比>=10", &大きい群)] {
        for 読み取り in 群.iter().take(群ごとに綴る例の数) {
            println!("    {名前}: {}", 読み取り.綴り());
        }
    }
}
