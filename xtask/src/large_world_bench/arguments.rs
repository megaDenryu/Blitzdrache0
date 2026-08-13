//! 大規模世界計測の条件をCLIから読む。全値の既定は最初の公式実測条件である。

use std::path::PathBuf;

mod value_read;

use value_read::{シーンを読む, 数の誤り, 有限値を読む, 正の数を読む, 正の有限値を読む};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct 計測指定 {
    pub(crate) アセットルート: PathBuf,
    pub(crate) シーン: String,
    pub(crate) フレーム数: u32,
    pub(crate) 先読み半径: u8,
    pub(crate) ram上限: u64,
    pub(crate) vram上限: u64,
    pub(crate) ワーカー本数: usize,
    pub(crate) 要求容量: usize,
    pub(crate) 完了容量: usize,
    pub(crate) 始点東: f64,
    pub(crate) 始点南: f64,
    pub(crate) 終点東: f64,
    pub(crate) 終点南: f64,
    pub(crate) 一フレーム移動量: f64,
    pub(crate) 計画だけ: bool,
}

impl Default for 計測指定 {
    fn default() -> Self {
        Self {
            アセットルート: PathBuf::from("target/large_world_generation_check/runtime_x"),
            シーン: "terrain_fox_tour".to_string(),
            フレーム数: 1_000,
            先読み半径: 8,
            ram上限: 512 * 1024 * 1024,
            vram上限: 512 * 1024 * 1024,
            ワーカー本数: 1,
            要求容量: 64,
            完了容量: 4,
            始点東: 0.0,
            始点南: -4_000.0,
            終点東: 0.0,
            終点南: 4_000.0,
            一フレーム移動量: 20.0,
            計画だけ: false,
        }
    }
}

pub(crate) fn 引数を読む(引数一覧: &[String]) -> Result<計測指定, String> {
    let mut 指定 = 計測指定::default();
    let mut 残り = 引数一覧.iter();
    while let Some(名前) = 残り.next() {
        if 名前 == "--print-plan" {
            指定.計画だけ = true;
            continue;
        }
        let 値 = 残り.next().ok_or_else(|| format!("{名前}の次に値が無い"))?;
        match 名前.as_str() {
            "--asset-root" => 指定.アセットルート = PathBuf::from(値),
            "--scene" => 指定.シーン = シーンを読む(値)?,
            "--frames" => 指定.フレーム数 = 正の数を読む(名前, 値)?,
            "--preload-radius" => 指定.先読み半径 = 値.parse().map_err(|_| 数の誤り(名前, 値))?,
            "--ram-limit" => 指定.ram上限 = 正の数を読む(名前, 値)?,
            "--vram-limit" => 指定.vram上限 = 正の数を読む(名前, 値)?,
            "--loader-workers" => 指定.ワーカー本数 = 正の数を読む(名前, 値)?,
            "--request-capacity" => 指定.要求容量 = 正の数を読む(名前, 値)?,
            "--completion-capacity" => 指定.完了容量 = 正の数を読む(名前, 値)?,
            "--route-start-east-meters" => 指定.始点東 = 有限値を読む(名前, 値)?,
            "--route-start-south-meters" => 指定.始点南 = 有限値を読む(名前, 値)?,
            "--route-end-east-meters" => 指定.終点東 = 有限値を読む(名前, 値)?,
            "--route-end-south-meters" => 指定.終点南 = 有限値を読む(名前, 値)?,
            "--route-meters-per-frame" => 指定.一フレーム移動量 = 正の有限値を読む(名前, 値)?,
            _ => return Err(format!("知らない引数である: {名前}")),
        }
    }
    if 指定.先読み半径 > 16 {
        return Err("--preload-radiusは16以下でなければならない".to_string());
    }
    Ok(指定)
}
