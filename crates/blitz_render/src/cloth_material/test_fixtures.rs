//! 布素材の検査が共有する材料: 一辺2(粒子4)の粒子バイト列、距離拘束と目標拘束と目標位置の1件のバイト列、材料の組み立て。

use blitz_math::秒;

use super::*;

pub(super) fn 拘束(a: u32, b: u32, 静止長: f32, コンプライアンス: f32) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(16);
    バイト列.extend_from_slice(&a.to_le_bytes());
    バイト列.extend_from_slice(&b.to_le_bytes());
    バイト列.extend_from_slice(&静止長.to_le_bytes());
    バイト列.extend_from_slice(&コンプライアンス.to_le_bytes());
    バイト列
}

pub(super) fn 目標拘束(粒子添字: u32, コンプライアンス: f32) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(8);
    バイト列.extend_from_slice(&粒子添字.to_le_bytes());
    バイト列.extend_from_slice(&コンプライアンス.to_le_bytes());
    バイト列
}

pub(super) fn 目標位置(位置: [f32; 3], 有効: f32) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(16);
    for 成分 in 位置 {
        バイト列.extend_from_slice(&成分.to_le_bytes());
    }
    バイト列.extend_from_slice(&有効.to_le_bytes());
    バイト列
}

pub(super) fn 区間(開始: u32, 本数: u32) -> 布の彩色の区間 {
    布の彩色の区間 { 開始, 本数 }
}

// 4粒子の粒子バイト列。位置は原点、逆質量は与えた値、速度は0である。
pub(super) fn 粒子バイト列(逆質量: f32) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(4 * 32);
    for _ in 0..4 {
        for 値 in [0.0f32, 0.0, 0.0, 逆質量, 0.0, 0.0, 0.0, 0.0] {
            バイト列.extend_from_slice(&値.to_le_bytes());
        }
    }
    バイト列
}

// 一辺2(粒子4)の材料。拘束は与えたバイト列をそのまま並べ、乗数は距離と目標の本数ぶんの零である。
pub(super) fn 目標拘束つきの材料(
    拘束一覧: &[Vec<u8>],
    色の区間一覧: Vec<布の彩色の区間>,
    目標拘束一覧: &[Vec<u8>],
    目標位置一覧: &[Vec<u8>],
) -> 布素材の材料 {
    let 刻み幅 = match 布の刻み幅::生成する(秒::生成する(1.0 / 60.0)) {
        Ok(刻み幅) => 刻み幅,
        Err(誤り) => panic!("検査の刻み幅が作れない: {誤り}"),
    };
    布素材の材料 {
        粒子バイト列: 粒子バイト列(1.0),
        拘束の引数バイト列: 拘束一覧.concat(),
        ラグランジュ乗数の初期バイト列: vec![0u8; (拘束一覧.len() + 目標拘束一覧.len()) * 4],
        色の区間一覧,
        目標拘束の引数バイト列: 目標拘束一覧.concat(),
        目標位置の初期バイト列: 目標位置一覧.concat(),
        目標の更新対応一覧: Vec::new(),
        インデックス一覧: Vec::new(),
        一辺粒子数: 2,
        定数: 布定数 {
            重力: [0.0, -9.8, 0.0],
            粒子間隔: 0.05,
            グリッド原点: [0.0; 3],
            速度減衰: 0.99,
            自己衝突: 布の自己衝突::行わない,
            刻み幅,
        },
    }
}

// 目標拘束を持たない材料。
pub(super) fn 材料(拘束一覧: &[Vec<u8>], 色の区間一覧: Vec<布の彩色の区間>) -> 布素材の材料 {
    目標拘束つきの材料(拘束一覧, 色の区間一覧, &[], &[])
}
