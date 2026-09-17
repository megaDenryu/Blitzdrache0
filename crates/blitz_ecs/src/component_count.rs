//! 個体構成要素の型ごとの件数と占有量: 計器と予算の会計が型ごとの量を読むための1行(契約26)。
//! 人が読む名前は `std::any::type_name` が返すRustの型の綴りであり、登録情報としての表示名は Issue #49 の範囲外である。

/// 1つの個体構成要素の型について、いま持っている件数と、置き場が確保しているバイト数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 個体構成要素の型ごとの件数と占有量 {
    型の名前: &'static str,
    件数: usize,
    占有量のバイト数: usize,
}

impl 個体構成要素の型ごとの件数と占有量 {
    pub(crate) fn 生成する(型の名前: &'static str, 件数: usize, 占有量のバイト数: usize) -> Self {
        Self {
            型の名前, 件数, 占有量のバイト数
        }
    }

    /// Rustの型の綴り。
    pub fn 型の名前(&self) -> &'static str {
        self.型の名前
    }

    /// この型を持つ個体の数。
    pub fn 件数(&self) -> usize {
        self.件数
    }

    /// 疎な配列と密な列が確保しているバイト数。
    pub fn 占有量のバイト数(&self) -> usize {
        self.占有量のバイト数
    }
}
