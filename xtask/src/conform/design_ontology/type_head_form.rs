//! 型の表記の先頭の形を読む純粋な関数。受け取るのは型の表記、返すのは先頭が裸のパスか、関連型の射影かである。
//! 自己変更の検査は、実装の対象の型と型の別名の右辺を名前で追う。先頭が関連型の射影(`<A as B>::C`)である表記は、指す型の名前を字面から読めない。
//! そのため正規形の検査(`name_traceable_form_assertion.rs`)がこれを違反にする。先頭を読む前に、参照と寿命と `mut` と、包む丸括弧・角括弧を剥がす。`&mut <A as B>::C` も先頭は裸のパスでないためである。
//! 型引数の中の射影(`Result<<A as B>::C, E>`)は剥がした先頭に現れないため、この関数は裸のパスと答える。型引数の中は名前で追う対象でないためである。
//! マクロの呼び出し(`名前!(..)`)はこの関数で見分けない。字句の木の側(`token_tree_gate/type_notation_macro_assertion.rs`)が、先頭に限らずどの深さでも違反にするためである。

use super::identifier_boundary::識別子の文字か;

/// 型の表記の先頭の形。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 型の表記の先頭 {
    裸のパス,
    関連型の射影,
}

impl 型の表記の先頭 {
    /// 型の表記の先頭の形を読む。
    pub fn 読む(表記: &str) -> Self {
        let 先頭 = 包みを剥がす(表記);
        if 先頭.starts_with('<') { Self::関連型の射影 } else { Self::裸のパス }
    }

    /// 名前で追えない先頭なら、その形を表す語。裸のパスなら無い。
    pub const fn 名前で追えない形(self) -> Option<&'static str> {
        match self {
            Self::裸のパス => None,
            Self::関連型の射影 => Some("関連型の射影"),
        }
    }
}

// 参照と寿命と `mut` と、包む丸括弧・角括弧を、剥がせなくなるまで剥がした残り。
fn 包みを剥がす(表記: &str) -> &str {
    let mut 残り = 表記.trim();
    loop {
        let 剥がした = 残り
            .strip_prefix(['&', '(', '['])
            .map(str::trim_start)
            .map(|後ろ| 後ろ.strip_prefix('\'').map_or(後ろ, |寿命| 寿命.trim_start_matches(識別子の文字か).trim_start()))
            .map(|後ろ| 後ろ.strip_prefix("mut").filter(|後ろ| 後ろ.starts_with(char::is_whitespace)).map_or(後ろ, str::trim_start));
        match 剥がした {
            Some(後ろ) if 後ろ != 残り => 残り = 後ろ,
            Some(_) | None => return 残り,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::型の表記の先頭;

    #[test]
    fn 参照と包みを剥がした先頭の形を読む() {
        for 表記 in ["規則", "crate::a::規則<T>", "Vec<規則>", "Result<<甲 as 乙>::丙, E>", "&mut 規則", "[規則]"] {
            assert_eq!(型の表記の先頭::読む(表記), 型の表記の先頭::裸のパス, "{表記}");
        }
        for 表記 in ["<甲 as 乙>::丙", "&mut <甲 as 乙>::丙", "&'a <甲 as 乙>::丙", "(<甲 as 乙>::丙)"] {
            assert_eq!(型の表記の先頭::読む(表記), 型の表記の先頭::関連型の射影, "{表記}");
        }
    }
}
