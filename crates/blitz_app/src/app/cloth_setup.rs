//! 布素材の構築(判断52)。blitz_simの布データをGPU境界のバイト列へ変換する。
//! アタッチ先の選択は`attach`、素材の組み立ては`material`、XPBDの参照比較の題材は`reference_preset`にある。数値はFoxのスケール(1単位約1cm)前提。

mod attach;
mod material;
mod preset;
mod reference_preset;

use preset::{マントを構築する, 吊るし布を構築する};

use super::cloth_reference::布の参照比較;

/// 布モードから組み上げた布の一式。参照比較の方式だけが、終了時にCPUの参照計算と突き合わせる材料を持つ。
pub(super) struct 布の構築物 {
    pub(super) 素材: blitz_render::布素材,
    pub(super) プリセット: 布プリセット,
    pub(super) 参照比較: Option<布の参照比較>,
}

/// 起動指定の布モードから布の一式を作る。受け取るのはモードと描画シーン、
/// 返すのは布を持たない起動では`None`である。モードごとにどの原型へ布を張るかを決めるのはこの工程だけであり、
/// 呼び出し元はモードの種類を1つも知らずに一式を受け取る。
pub(super) fn 布モードから構築する(
    布モード: crate::cli::布モード,
    描画シーン: &blitz_render::描画シーン素材,
) -> Result<Option<布の構築物>, crate::error::起動エラー> {
    let (素材, プリセット, 参照比較) = match 布モード {
        crate::cli::布モード::なし => return Ok(None),
        crate::cli::布モード::吊るし布 => {
            let (素材, プリセット) = 吊るし布を構築する()?;
            (素材, プリセット, None)
        }
        crate::cli::布モード::マント => {
            let (素材, プリセット) = マントを構築する(描画シーン.先頭の描画対象().最詳細段の頂点一覧())?;
            (素材, プリセット, None)
        }
        crate::cli::布モード::XPBD参照比較 { コンプライアンス } => {
            let (素材, プリセット, 参照比較) = reference_preset::参照比較の布を構築する(コンプライアンス)?;
            (素材, プリセット, Some(参照比較))
        }
    };
    Ok(Some(布の構築物 {
        素材, プリセット, 参照比較
    }))
}

pub(super) const 一辺粒子数: u32 = 32;
const マント間隔: f32 = 3.2;
const 吊るし間隔: f32 = 0.05;
const 総質量: f32 = 4.0;

/// 掴み介入のカーソル→ワールド写像: 目標 = 中心 + 横基底*x_ndc + 縦基底*y_ndc(判断53の簡易写像)。
pub(super) struct 掴み写像 {
    pub(super) 中心: [f32; 3],
    pub(super) 横基底: [f32; 3],
    pub(super) 縦基底: [f32; 3],
}

/// 布シナリオごとの実行時パラメータ(判断56)。カプセルNoneはキャラ衝突なし。
pub(super) struct 布プリセット {
    pub(super) カプセル: Option<([f32; 3], [f32; 3], f32)>,
    pub(super) 掴み: 掴み写像,
}

/// 前提: 帯数は上端行の粒子数(高々数百)でu16に収まる。
fn 帯数を実数へ変換する(帯数: usize) -> f32 {
    f32::from(u16::try_from(帯数).unwrap_or_else(|_| panic!("帯数がu16に収まらない: {帯数}")))
}

fn 添字をusizeへ変換する(値: u32) -> usize {
    usize::try_from(値).unwrap_or_else(|_| panic!("頂点添字がusizeに収まらない: {値}"))
}
