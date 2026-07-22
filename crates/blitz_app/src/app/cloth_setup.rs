//! 布素材の構築(判断52)。blitz_simの布データをGPU境界のバイト列へ変換する。
//! アタッチ先の選択は`attach`にある。数値はFoxのスケール(1単位約1cm)前提。

mod attach;
mod preset;

pub(super) use preset::{マントを構築する, 吊るし布を構築する};

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
fn 帯数を実数へ(帯数: usize) -> f32 {
    f32::from(u16::try_from(帯数).unwrap_or_else(|_| panic!("帯数がu16に収まらない: {帯数}")))
}

fn 添字をusizeへ(値: u32) -> usize {
    usize::try_from(値).unwrap_or_else(|_| panic!("頂点添字がusizeに収まらない: {値}"))
}
