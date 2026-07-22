//! 布の毎フレーム入力(判断53・54・56)。カプセルと掴み写像はシナリオごとの布プリセットから取る。
//! 掴む介入: マウス右押下中、布の下端角の粒子をカーソル位置から写した目標へ毎フレーム固定し、
//! 離した最初のフレームで解放する(掴み状態バッファを持たない設計)。

use blitz_render::布フレーム入力;
use blitz_sim::介入;

use super::cloth_setup::布プリセット;
use super::アプリ;

/// 掴み対象は布の下端行の左端(吊るし布・マントとも上端行は固定で掴めない)。
const 掴み粒子添字: u32 = (super::cloth_setup::一辺粒子数 - 1) * super::cloth_setup::一辺粒子数;

impl アプリ {
    pub(super) fn 布フレーム入力を作る(&mut self) -> Option<布フレーム入力> {
        let Some(プリセット) = &self.布プリセット else {
            return None;
        };
        let (カプセル端点a, カプセル端点b, カプセル半径) = プリセット.カプセル.unwrap_or(([0.0; 3], [0.0; 3], 0.0));
        let 介入一覧 = 掴み介入(self.入力状態.掴み操作(), self.window.as_ref(), プリセット, &mut self.掴み中だった);
        let 介入件数 = u32::try_from(介入一覧.len()).unwrap_or_else(|_| panic!("介入件数がu32に収まらない"));
        Some(布フレーム入力 {
            カプセル端点a,
            カプセル端点b,
            カプセル半径,
            介入バイト列: blitz_sim::バイト列にする(&介入一覧),
            介入件数,
        })
    }
}

fn 掴み介入(
    掴み: Option<(f32, f32)>, window: Option<&winit::window::Window>, プリセット: &布プリセット, 掴み中だった: &mut bool
) -> Vec<介入> {
    match 掴み.zip(window.map(|w| w.inner_size())) {
        Some(((px, py), 寸法)) => {
            *掴み中だった = true;
            vec![介入::掴む {
                粒子添字: 掴み粒子添字,
                目標位置: 目標位置へ写す(プリセット, px, py, 寸法.width, 寸法.height),
            }]
        }
        None if *掴み中だった => {
            *掴み中だった = false;
            vec![介入::離す {
                粒子添字: 掴み粒子添字
            }]
        }
        None => Vec::new(),
    }
}

/// カーソル位置(物理px)をプリセットの写像(中心+横基底*x_ndc+縦基底*y_ndc)でワールドへ写す。
/// レイキャストは導入しない(判断53の最小実装)。
fn 目標位置へ写す(プリセット: &布プリセット, px: f32, py: f32, 幅: u32, 高さ: u32) -> [f32; 3] {
    let x_ndc = (px / 実数へ(幅)) * 2.0 - 1.0;
    let y_ndc = 1.0 - (py / 実数へ(高さ)) * 2.0;
    let 写像 = &プリセット.掴み;
    [
        写像.中心[0] + 写像.横基底[0] * x_ndc + 写像.縦基底[0] * y_ndc,
        写像.中心[1] + 写像.横基底[1] * x_ndc + 写像.縦基底[1] * y_ndc,
        写像.中心[2] + 写像.横基底[2] * x_ndc + 写像.縦基底[2] * y_ndc,
    ]
}

/// 前提: ウィンドウ寸法は実運用で65535を超えない(draw_commandsのu16経由変換と同じ根拠)。
fn 実数へ(値: u32) -> f32 {
    f32::from(u16::try_from(値).unwrap_or_else(|_| panic!("ウィンドウ寸法がu16に収まらない: {値}")))
}
