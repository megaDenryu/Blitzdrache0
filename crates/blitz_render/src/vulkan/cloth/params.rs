//! 布シミュ定数UBO(ClothParams、96バイト)のバイト列化。レイアウトは
//! shaders/cloth_step.slang冒頭の仕様と一致させる(std140。全メンバがvec4/uint4のため詰め物なし)。

use crate::cloth_material::{布の固定刻みの秒, 布の彩色の区間, 布定数};
use crate::frame_input::布フレーム入力;

pub(super) const バイト長: usize = 96;
pub(super) const 介入上限件数: u32 = 64;

pub(super) struct 固定部 {
    pub(super) 定数: 布定数,
    pub(super) 粒子数: u32,
    pub(super) 拘束の数: u32,
    pub(super) 色の区間一覧: Vec<布の彩色の区間>, // 彩色で並べ替えた拘束の並びの、色ごとの区間。生成後は変わらない
    pub(super) 一辺粒子数: u32,
    pub(super) アタッチ件数: u32,
    pub(super) 既定逆質量: f32,
}

pub(super) fn バイト列にする(固定: &固定部, 入力: &布フレーム入力) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(バイト長);
    // gravityDt
    f32列を積む(
        &mut バイト列,
        &[固定.定数.重力[0], 固定.定数.重力[1], 固定.定数.重力[2], 布の固定刻みの秒],
    );
    // gridOrigin + セル寸法(=粒子間隔)
    f32列を積む(
        &mut バイト列,
        &[
            固定.定数.グリッド原点[0],
            固定.定数.グリッド原点[1],
            固定.定数.グリッド原点[2],
            固定.定数.粒子間隔,
        ],
    );
    // capsuleA + 半径
    f32列を積む(
        &mut バイト列,
        &[入力.カプセル端点a[0], 入力.カプセル端点a[1], 入力.カプセル端点a[2], 入力.カプセル半径],
    );
    // capsuleB + 既定逆質量
    f32列を積む(
        &mut バイト列,
        &[入力.カプセル端点b[0], 入力.カプセル端点b[1], 入力.カプセル端点b[2], 固定.既定逆質量],
    );
    // counts { 粒子数, 一辺, 介入件数, アタッチ件数 }
    for 値 in [固定.粒子数, 固定.一辺粒子数, 入力.介入件数, 固定.アタッチ件数] {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
    // misc { 衝突半径(粒子間隔の0.4倍), 刻み幅の2乗の逆数, 速度減衰, 予備 }
    f32列を積む(
        &mut バイト列,
        &[
            固定.定数.粒子間隔 * 0.4,
            1.0 / (布の固定刻みの秒 * 布の固定刻みの秒),
            固定.定数.速度減衰,
            0.0,
        ],
    );
    バイト列
}

fn f32列を積む(バイト列: &mut Vec<u8>, 値一覧: &[f32]) {
    for 値 in 値一覧 {
        バイト列.extend_from_slice(&値.to_le_bytes());
    }
}
