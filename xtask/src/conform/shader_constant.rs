//! CPU正本とslangの写しが同じ値を持つべき定数の突き合わせ。
//! 受け取るのは無し(対象の組は表が持つ)、返すのは値が食い違った組の違反一覧である。
//!
//! 正本と写しはコメントでしか結ばれておらず、片方だけを直した食い違いは走らせても絵に出ない。
//! とりわけ「CPUは型付きエラー、GPUは異常値」という同じ境界を両側が持つ定数は、値がずれると
//! 検出の境界そのものがずれるため、GPUとCPUの一致検査でも捉えられない。ここが機械的に見る。

use std::path::{Path, PathBuf};

use super::violation::違反;

/// 突き合わせる定数の組。正本の行と写しの行を、それぞれ宣言の前置きで見つける。
struct 定数の組 {
    正本パス: &'static str,
    正本の前置き: &'static str,
    写しパス: &'static str,
    写しの前置き: &'static str,
}

const 定数一覧: [定数の組; 3] = [
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/integration/sun_visibility.rs",
        正本の前置き: "pub(in crate::atmosphere) const 遮蔽球を縮める半径メートル: f64 = ",
        写しパス: "shaders/atmosphere_scatter.slang",
        写しの前置き: "static const float shadowSphereShrinkMeters = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/integration/multiscatter_series.rs",
        正本の前置き: "pub(in crate::atmosphere) const 公比の上限: f64 = ",
        写しパス: "shaders/atmosphere_multiscatter.slang",
        写しの前置き: "static const float multiScatterRatioLimit = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/integration/skyview_march.rs",
        正本の前置き: "pub(in crate::atmosphere) const 標本区間数: u32 = ",
        写しパス: "shaders/atmosphere_skyview_march.slang",
        写しの前置き: "static const uint skyViewStepCount = ",
    },
];

pub fn 全定数を検査する() -> Result<Vec<違反>, String> {
    let mut 違反一覧 = Vec::new();
    for 組 in &定数一覧 {
        let 正本 = 値を読む(組.正本パス, 組.正本の前置き)?;
        let 写し = 値を読む(組.写しパス, 組.写しの前置き)?;
        if 正本 != 写し {
            違反一覧.push(違反::ファイル単位(
                PathBuf::from(組.写しパス),
                format!(
                    "{}の写しが正本と食い違う(正本{正本}・写し{写し}・正本は{})",
                    組.写しの前置き.trim(),
                    組.正本パス
                ),
            ));
        }
    }
    Ok(違反一覧)
}

/// 前置きで始まる行を1本見つけ、そのセミコロンまでを数として読む。
/// 見つからない場合を違反でなく失敗として扱うのは、宣言の消失を「一致した」と読み替えないためである。
fn 値を読む(パス: &str, 前置き: &str) -> Result<f64, String> {
    let 内容 = std::fs::read_to_string(Path::new(パス)).map_err(|誤り| format!("{パス}の読み取りに失敗した: {誤り}"))?;
    let 行 = 内容
        .lines()
        .find(|行| 行.trim_start().starts_with(前置き))
        .ok_or_else(|| format!("{パス}に「{前置き}」で始まる行が無い"))?;
    let 数値 = 行.trim_start().trim_start_matches(前置き).split(';').next().unwrap_or("").trim();
    数値
        .parse::<f64>()
        .map_err(|誤り| format!("{パス}の「{前置き}」の値を数として読めない: {数値}({誤り})"))
}
