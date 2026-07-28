//! 段ごとに1つずつ数える4本の並行した列。触れるのはこの4本だけであり、どれも常に段数ぶんの長さを持つ。
//! 4本をまとめて1つの型が持つのは、長さが揃っていることと段番号がその長さの内側に収まることが
//! この4本にまたがる不変条件であり、どれか1本だけを外から伸ばされると破れるためである。

#[derive(Default)]
pub(super) struct 段別集計 {
    個体数: Vec<u32>,
    可視数: Vec<u32>,
    可視書込位置: Vec<u32>,
    不可視書込位置: Vec<u32>,
}

impl 段別集計 {
    pub(super) fn 群を始める(&mut self, 段数: usize) {
        for 列 in [&mut self.個体数, &mut self.可視数, &mut self.可視書込位置, &mut self.不可視書込位置] {
            列.clear();
            列.resize(段数, 0);
        }
    }

    pub(super) fn 段数(&self) -> usize {
        self.個体数.len()
    }

    pub(super) fn 個体を数える(&mut self, 段番号: usize, 可視: bool) {
        *枠(&mut self.個体数, 段番号) += 1;
        if 可視 {
            *枠(&mut self.可視数, 段番号) += 1;
        }
    }

    /// その段の個体数と可視数を返し、次の書き出しのための2つの書込位置を据える。
    pub(super) fn 段の範囲を据える(&mut self, 段番号: usize, 開始: u32) -> (u32, u32) {
        let (個体数, 可視数) = (*枠(&mut self.個体数, 段番号), *枠(&mut self.可視数, 段番号));
        *枠(&mut self.可視書込位置, 段番号) = 開始;
        *枠(&mut self.不可視書込位置, 段番号) = 開始 + 可視数;
        (可視数, 個体数)
    }

    pub(super) fn 書込位置を取り出して進める(&mut self, 段番号: usize, 可視: bool) -> u32 {
        let 列 = if 可視 {
            &mut self.可視書込位置
        } else {
            &mut self.不可視書込位置
        };
        let 枠 = 枠(列, 段番号);
        let 位置 = *枠;
        *枠 += 1;
        位置
    }
}

/// 段番号は原型の段数の内側に収まる。破れは呼び出し側の不変条件違反である。
fn 枠(一覧: &mut [u32], 段番号: usize) -> &mut u32 {
    match 一覧.get_mut(段番号) {
        Some(値) => 値,
        None => panic!("段別の集計が原型の段数ぶんの長さを持つという不変条件に違反した: 段番号{段番号}"),
    }
}
