//! エディターチャンクの版付きソース。高さ格子と建物配置を1つの明示的な入口から参照させる。

use serde::Serialize;

use super::error::書き出しエラー;
use crate::resource::建物の配置;

#[cfg(test)]
mod tests;

pub(super) const 形式版: u32 = 1;

#[derive(Serialize)]
pub(super) struct エディターチャンクソース {
    形式版: u32,
    高さ格子: String,
    建物配置一覧: Vec<建物配置ソース>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct 建物配置ソース {
    配置識別子: String,
    建物定義ID: String,
    チャンク原点からの東メートル: f32,
    チャンク原点からの南メートル: f32,
    向きラジアン: f32,
}

impl エディターチャンクソース {
    pub(super) fn 組み立てる(
        高さ格子: String, 建物一覧: Vec<建物の配置>, チャンク一辺メートル: f32
    ) -> Result<Self, 書き出しエラー> {
        let チャンク一辺 = f64::from(チャンク一辺メートル);
        // 編集モデルは各チャンクの中心を(0,0)とし、実行時形式は各チャンクの南西端を
        // (0,0)とする。境界で半辺だけ平行移動し、チャンク番号は混ぜない。
        let 原点の東 = -チャンク一辺 * 0.5;
        let 原点の南 = -チャンク一辺 * 0.5;
        let 建物配置一覧 = 建物一覧
            .into_iter()
            .map(|建物| 建物配置ソース::エディター座標から変換する(建物, 原点の東, 原点の南, チャンク一辺))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            形式版,
            高さ格子,
            建物配置一覧,
        })
    }

    pub(super) fn 整形済みjsonを作る(&self) -> Result<Vec<u8>, 書き出しエラー> {
        let mut バイト列 = serde_json::to_vec_pretty(self)?;
        バイト列.push(b'\n');
        Ok(バイト列)
    }
}

impl 建物配置ソース {
    fn エディター座標から変換する(
        建物: 建物の配置,
        原点の東: f64,
        原点の南: f64,
        チャンク一辺: f64,
    ) -> Result<Self, 書き出しエラー> {
        let 東 = 建物.位置.x - 原点の東;
        let 南 = 建物.位置.z - 原点の南;
        if !(0.0..=チャンク一辺).contains(&東) || !(0.0..=チャンク一辺).contains(&南) {
            return Err(書き出しエラー::建物が所有チャンクの外にある {
                建物識別子: 建物.識別子,
                局所の東: 東,
                局所の南: 南,
                チャンク一辺,
            });
        }
        Ok(Self {
            配置識別子: 建物.識別子,
            建物定義ID: 建物.建物定義ID,
            チャンク原点からの東メートル: f32へ狭める("建物の局所東座標", 東)?,
            チャンク原点からの南メートル: f32へ狭める("建物の局所南座標", 南)?,
            向きラジアン: f32へ狭める("建物の向き", 建物.向きラジアン)?,
        })
    }
}

fn f32へ狭める(名前: &str, 値: f64) -> Result<f32, 書き出しエラー> {
    if !値.is_finite() || 値 < f64::from(f32::MIN) || 値 > f64::from(f32::MAX) {
        return Err(書き出しエラー::数値変換に失敗(format!("{名前}{値}がf32へ収まらない")));
    }
    値.to_string()
        .parse::<f32>()
        .ok()
        .filter(|狭めた| 狭めた.is_finite())
        .ok_or_else(|| 書き出しエラー::数値変換に失敗(format!("{名前}{値}をf32へ狭められない")))
}
