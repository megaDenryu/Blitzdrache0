//! 布シミュレーションのGPU境界型(判断52・54)。blitz_simが生成したバイト列・一覧を検証付きで
//! 保持する。バイト列のレイアウトはshaders/cloth_step.slang冒頭のバインディング表と一致する
//! (粒子32バイト・隣接拘束64バイト/粒子)。

use thiserror::Error;

/// 布の物性・空間定数。単位はシーンのワールド単位(Foxは1単位約1cm)で、値はアプリが与える。
#[derive(Debug, Clone, Copy)]
pub struct 布定数 {
    pub 重力: [f32; 3],
    pub 粒子間隔: f32,
    pub グリッド原点: [f32; 3],
    pub 過緩和: f32,
    pub 速度減衰: f32,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum 布素材エラー {
    #[error("一辺粒子数{0}が2未満だった")]
    一辺粒子数不足(u32),
    #[error("{名前}のバイト列長{実際}が期待{期待}と一致しない")]
    バイト列長不一致 { 名前: &'static str, 期待: usize, 実際: usize },
    #[error("{名前}の布粒子添字{添字}が粒子数{粒子数}の範囲外だった")]
    布粒子添字範囲外 { 名前: &'static str, 添字: u32, 粒子数: u32 },
}

/// 布1枚ぶんのGPU入力。スキン頂点添字(アタッチ対応の第2要素)の検証は、
/// スキン済み頂点数を知るレンダラー生成時に行う。
#[derive(Debug, Clone)]
pub struct 布素材 {
    pub(crate) 粒子バイト列: Vec<u8>,
    pub(crate) 隣接バイト列: Vec<u8>,
    pub(crate) インデックス一覧: Vec<u32>,
    pub(crate) アタッチ対応一覧: Vec<[u32; 2]>,
    pub(crate) 粒子数: u32,
    pub(crate) 一辺粒子数: u32,
    pub(crate) 定数: 布定数,
}

impl 布素材 {
    pub fn 生成する(
        粒子バイト列: Vec<u8>,
        隣接バイト列: Vec<u8>,
        インデックス一覧: Vec<u32>,
        アタッチ対応一覧: Vec<[u32; 2]>,
        一辺粒子数: u32,
        定数: 布定数,
    ) -> Result<Self, 布素材エラー> {
        if 一辺粒子数 < 2 {
            return Err(布素材エラー::一辺粒子数不足(一辺粒子数));
        }
        let 粒子数 = 一辺粒子数 * 一辺粒子数;
        let 粒子数usize = usize::try_from(粒子数).unwrap_or_else(|_| panic!("粒子数がusizeに収まらない"));
        長さを検証する("粒子バイト列", 粒子バイト列.len(), 粒子数usize * 32)?;
        長さを検証する("隣接バイト列", 隣接バイト列.len(), 粒子数usize * 64)?;
        for &添字 in &インデックス一覧 {
            if 添字 >= 粒子数 {
                return Err(布素材エラー::布粒子添字範囲外 {
                    名前: "インデックス一覧",
                    添字,
                    粒子数,
                });
            }
        }
        for 対応 in &アタッチ対応一覧 {
            if 対応[0] >= 粒子数 {
                return Err(布素材エラー::布粒子添字範囲外 {
                    名前: "アタッチ対応一覧",
                    添字: 対応[0],
                    粒子数,
                });
            }
        }
        Ok(Self {
            粒子バイト列,
            隣接バイト列,
            インデックス一覧,
            アタッチ対応一覧,
            粒子数,
            一辺粒子数,
            定数,
        })
    }
}

fn 長さを検証する(名前: &'static str, 実際: usize, 期待: usize) -> Result<(), 布素材エラー> {
    if 実際 != 期待 {
        return Err(布素材エラー::バイト列長不一致 { 名前, 期待, 実際 });
    }
    Ok(())
}
