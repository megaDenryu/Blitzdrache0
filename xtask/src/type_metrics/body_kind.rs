//! 本体の種別ごとに、本体内で数える行の判定と、数え終わった結果を何の観測にするかを決める。

use super::member_line;
use super::observation::観測;

pub enum 本体種別 {
    構造体,
    実装,
}

impl 本体種別 {
    pub fn 数える対象か(&self, 行: &str) -> bool {
        match self {
            Self::構造体 => member_line::フィールド定義か(行),
            Self::実装 => member_line::メソッド定義か(行),
        }
    }

    pub fn 観測にする(&self, 型名: String, 件数: usize) -> 観測 {
        match self {
            Self::構造体 => 観測::型定義 {
                型名, フィールド数: 件数
            },
            Self::実装 => 観測::実装ブロック {
                型名, メソッド数: 件数
            },
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 種別ごとに数える行が変わる() {
        assert!(本体種別::構造体.数える対象か("    件数: usize,"));
        assert!(!本体種別::実装.数える対象か("    件数: usize,"));
        assert!(本体種別::実装.数える対象か("    pub fn 数える(&self) {"));
    }

    #[test]
    fn 種別ごとに観測の種類が変わる() {
        let 観測 = 本体種別::構造体.観測にする("台帳".to_string(), 3);
        assert!(matches!(観測, 観測::型定義 { フィールド数: 3, .. }));
    }
}
