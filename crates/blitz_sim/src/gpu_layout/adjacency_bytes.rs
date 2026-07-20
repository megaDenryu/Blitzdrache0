//! 隣接拘束リストのGPU表現(判断49追記・判断54): 1粒子64バイト(std430) =
//! 8スロット×(相手粒子添字u32 + 静止長f32)。空きスロットは相手添字0xFFFFFFFFで埋まる
//! (`隣接拘束エントリ::空き`)。gather方式の拘束反復がスレッドごとに読む。

use crate::cloth::布データ;

const スロット数: usize = 8;
const スロット1件のバイト数: usize = 8;
const 粒子1件のバイト数: usize = スロット数 * スロット1件のバイト数;

pub fn 隣接拘束バイト列にする(布: &布データ) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(布.隣接拘束一覧.len() * 粒子1件のバイト数);
    for スロット一覧 in &布.隣接拘束一覧 {
        for エントリ in スロット一覧 {
            バイト列.extend_from_slice(&エントリ.相手粒子添字.to_le_bytes());
            バイト列.extend_from_slice(&エントリ.静止長.to_le_bytes());
        }
    }
    バイト列
}
