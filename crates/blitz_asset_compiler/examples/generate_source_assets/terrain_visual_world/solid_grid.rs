//! 立体1つぶんの頂点と三角形の入れ物。担当するのは「位置と法線を同じ並びで持ち、三角形の添字がその並びの内側を指す」ことだけである。
//! 添字は立体ごとに0から始まる局所の番号であり、連結して1つのメッシュにする側がずらし量を加える。
//! 球を作る工程と台座を作る工程がどちらもこの入れ物へ書き込むため、2つの立体の並べ方が1通りに決まる。

pub(super) struct 立体の格子 {
    位置一覧: Vec<[f32; 3]>,
    法線一覧: Vec<[f32; 3]>,
    添字一覧: Vec<u16>,
}

impl 立体の格子 {
    pub(super) fn 空を作る() -> Self {
        Self {
            位置一覧: Vec::new(),
            法線一覧: Vec::new(),
            添字一覧: Vec::new(),
        }
    }

    pub(super) fn 頂点を足す(&mut self, 位置: [f32; 3], 法線: [f32; 3]) {
        self.位置一覧.push(位置);
        self.法線一覧.push(法線);
    }

    pub(super) fn 三角形を足す(&mut self, 添字: [u16; 3]) {
        self.添字一覧.extend_from_slice(&添字);
    }

    pub(super) fn 頂点数(&self) -> usize {
        self.位置一覧.len()
    }

    pub(super) fn 位置一覧(&self) -> &[[f32; 3]] {
        &self.位置一覧
    }

    pub(super) fn 法線一覧(&self) -> &[[f32; 3]] {
        &self.法線一覧
    }

    pub(super) fn 添字一覧(&self) -> &[u16] {
        &self.添字一覧
    }
}
