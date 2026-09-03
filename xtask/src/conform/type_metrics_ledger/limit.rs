//! 台帳へ登録した1つの型の上限値。計測が返す3つの指標と同じ形を持ち、照合の相手になる。

pub struct 型ごとの上限 {
    pub 型名: &'static str,
    pub 実装ファイル数: usize,
    pub フィールド数: usize,
    pub メソッド総数: usize,
}

impl 型ごとの上限 {
    pub const fn 生成する(型名: &'static str, 実装ファイル数: usize, フィールド数: usize, メソッド総数: usize) -> Self {
        Self {
            型名,
            実装ファイル数,
            フィールド数,
            メソッド総数,
        }
    }
}
