//! 材質見本の立体を1つのメッシュへ連結し、glTFのバッファバイト列へ直列化する工程。受け取るのは球と台座の宣言、
//! 返すのは連結した頂点と、立体ごとの三角形の区間である。
//!
//! 立体ごとに三角形の区間を分けるのは、球と台座が別々の材質を持ち、プリミティブの境目がそのまま材質の境目になるためである。
//! 区間の並び順(位置→法線→インデックス)と各区間のバイト長は`gltf_json`のbufferViewsへ手で一致させている。
//! テクスチャ座標と接線を書き出さないのは、どの材質も画像を1枚も持たず、読み込み側が欠如を計算で埋めるためである。

use super::sample_bodies::{台座の半奥行き, 台座の半幅, 台座の深さ, 球の一覧};
use super::solid_grid::立体の格子;
use super::{pedestal_mesh, sphere_mesh};

/// 立体1つが占める三角形の区間。ずらし量も個数も添字の個数で数える。
pub(super) struct 立体の区間 {
    pub(super) 添字ずらし量: usize,
    pub(super) 添字数: usize,
}

pub(super) struct 連結した立体 {
    位置一覧: Vec<[f32; 3]>,
    法線一覧: Vec<[f32; 3]>,
    添字一覧: Vec<u16>,
    pub(super) 区間一覧: Vec<立体の区間>,
}

/// 球を宣言の順に並べ、最後に台座を足す。この順がそのままプリミティブと材質の並びになる。
pub(super) fn 組み立てる() -> 連結した立体 {
    let mut 連結 = 連結した立体 {
        位置一覧: Vec::new(),
        法線一覧: Vec::new(),
        添字一覧: Vec::new(),
        区間一覧: Vec::new(),
    };
    for 球 in &球の一覧() {
        連結.立体を足す(&sphere_mesh::球を作る(球.中心, 球.半径));
    }
    連結.立体を足す(&pedestal_mesh::台座を作る(台座の半幅, 台座の半奥行き, 台座の深さ));
    連結
}

impl 連結した立体 {
    fn 立体を足す(&mut self, 格子: &立体の格子) {
        let ずらし量 = u16::try_from(self.位置一覧.len()).unwrap_or_else(|_| panic!("材質見本の立体の頂点番号がu16に収まらない"));
        let 添字ずらし量 = self.添字一覧.len();
        self.位置一覧.extend_from_slice(格子.位置一覧());
        self.法線一覧.extend_from_slice(格子.法線一覧());
        self.添字一覧.extend(格子.添字一覧().iter().map(|添字| 添字 + ずらし量));
        self.区間一覧.push(立体の区間 {
            添字ずらし量,
            添字数: 格子.添字一覧().len(),
        });
    }

    pub(super) fn 頂点数(&self) -> usize {
        self.位置一覧.len()
    }

    pub(super) fn 位置のバイト長(&self) -> usize {
        self.位置一覧.len() * 12
    }

    pub(super) fn 法線のバイト長(&self) -> usize {
        self.位置のバイト長()
    }

    pub(super) fn インデックスのバイト長(&self) -> usize {
        self.添字一覧.len() * 2
    }

    /// 位置アクセサが要る値域。glTFは全成分の最小と最大の宣言を求める。
    pub(super) fn 位置の値域(&self) -> ([f32; 3], [f32; 3]) {
        let mut 最小 = [f32::MAX; 3];
        let mut 最大 = [f32::MIN; 3];
        for 位置 in &self.位置一覧 {
            for 軸 in 0..3 {
                最小[軸] = 最小[軸].min(位置[軸]);
                最大[軸] = 最大[軸].max(位置[軸]);
            }
        }
        (最小, 最大)
    }

    pub(super) fn バッファバイト列を作る(&self) -> Vec<u8> {
        let mut バイト列 = Vec::with_capacity(self.位置のバイト長() * 2 + self.インデックスのバイト長());
        for 一覧 in [&self.位置一覧, &self.法線一覧] {
            for 値 in 一覧 {
                for 成分 in 値 {
                    バイト列.extend_from_slice(&成分.to_le_bytes());
                }
            }
        }
        for 添字 in &self.添字一覧 {
            バイト列.extend_from_slice(&添字.to_le_bytes());
        }
        バイト列
    }
}
