//! 本体の種別ごとに、本体内で数える行の判定と、数え終わった結果を何の観測にするかを決める。

use super::declaration_amount::宣言の分量;
use super::member_line;
use super::observation::観測;
use super::type_path::自己型の経路;

pub enum 本体種別 {
    構造体,
    列挙,
    実装,
}

impl 本体種別 {
    pub fn 数える対象か(&self, 行: &str) -> bool {
        match self {
            Self::構造体 => member_line::フィールド定義か(行),
            Self::列挙 => member_line::列挙の枝の開始か(行),
            Self::実装 => member_line::メソッド定義か(行),
        }
    }

    /// 宣言の綴りは、構造体と列挙なら型名そのものであり、implブロックなら実装対象として書かれた経路である。
    pub fn 観測にする(&self, 宣言の綴り: String, 件数: usize) -> 観測 {
        match self {
            Self::構造体 => 観測::型定義 {
                型名: 宣言の綴り,
                分量: 宣言の分量::構造体のフィールド数(件数),
            },
            Self::列挙 => 観測::型定義 {
                型名: 宣言の綴り,
                分量: 宣言の分量::列挙の枝数(件数),
            },
            Self::実装 => 観測::実装ブロック {
                自己型の経路: 自己型の経路::綴りから生成する(&宣言の綴り),
                メソッド数: 件数,
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
        assert!(本体種別::列挙.数える対象か("    値なし,"));
        assert!(!本体種別::列挙.数える対象か("    #[error(\"説明\")]"));
    }

    #[test]
    fn 種別ごとに観測の種類が変わる() {
        let 観測 = 本体種別::構造体.観測にする("台帳".to_string(), 3);
        assert!(matches!(
            観測,
            観測::型定義 {
                分量: 宣言の分量::構造体のフィールド数(3),
                ..
            }
        ));
        let 観測 = 本体種別::列挙.観測にする("観測".to_string(), 2);
        assert!(matches!(
            観測,
            観測::型定義 {
                分量: 宣言の分量::列挙の枝数(2),
                ..
            }
        ));
    }
}
