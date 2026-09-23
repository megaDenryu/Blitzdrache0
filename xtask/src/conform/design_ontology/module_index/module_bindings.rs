//! 1つのモジュールの名前の束縛。モジュールの索引(`module_index.rs`)がモジュールごとに1つ持ち、そのモジュールの直下に書いた型の定義と `use` の項目と条件付きの名前と、
//! マクロの呼び出しの中に書いた `use` と項目が隠しうる名前の控え(`module_index/hiding_names.rs`)を持つ。名前を解く工程が問う、あるモジュールでその名前がどう束縛されうるかに答える。
//! 条件付きの名前とは、条件付きの属性(`#[cfg(..)]`)を持つ直下の宣言と `use` が束縛する名前である。条件付きの宣言か明示の取り込みは消える場合があり、glob の取り込みを隠さない場合があるためこれを控える。

use std::collections::HashSet;

use super::super::declaration_prefix::属性と可視性を読み飛ばす;
use super::super::line_matching::先頭の識別子;
use super::super::use_resolution::取り込みの項目;
use super::file_lexical_position::項目の宣言の名前;
use super::hiding_names::隠しうる名前の控え;

#[derive(Default)]
pub struct モジュールの名前の束縛 {
    型の名前一覧: HashSet<String>,           // 直下の `struct`・`enum` の名前
    取り込みの項目一覧: Vec<取り込みの項目>, // 直下の `use` の項目
    条件付きの名前一覧: HashSet<String>,     // 条件付きの属性を持つ直下の宣言と `use` が束縛する名前
    マクロの呼び出しの中の名前: 隠しうる名前の控え,
}

impl モジュールの名前の束縛 {
    /// 直下の1行を足す。`struct`・`enum` の定義ならその型の名前を、条件付きの宣言ならその名前を控える。
    pub fn 直下の行を足す(&mut self, 行: &str, 条件付きか: bool) {
        self.型の名前一覧.extend(型の定義の名前(行));
        if 条件付きか {
            self.条件付きの名前一覧.extend(項目の宣言の名前(行));
        }
    }

    /// 直下の `use` の項目を足す。条件付きなら、その項目が名乗る名前も控える。
    pub fn 直下の取り込みを足す(&mut self, 項目: 取り込みの項目, 条件付きか: bool) {
        if 条件付きか {
            self.条件付きの名前一覧.insert(項目.名乗る名前().to_string());
        }
        self.取り込みの項目一覧.push(項目);
    }

    /// マクロの呼び出しの中に書いた行と `use` の項目を控える先。
    pub fn マクロの呼び出しの中の名前を控える(&mut self) -> &mut 隠しうる名前の控え {
        &mut self.マクロの呼び出しの中の名前
    }

    pub fn 型を定義しているか(&self, 型名: &str) -> bool {
        self.型の名前一覧.contains(型名)
    }

    pub fn 取り込みの項目一覧(&self) -> &[取り込みの項目] {
        &self.取り込みの項目一覧
    }

    pub fn 条件付きで束縛しているか(&self, 名前: &str) -> bool {
        self.条件付きの名前一覧.contains(名前)
    }

    /// マクロの呼び出しの中に書いた項目と `use` が持ち込みうる名前の控え。
    pub fn マクロの呼び出しの中の名前(&self) -> &隠しうる名前の控え {
        &self.マクロの呼び出しの中の名前
    }
}

// `struct 名` か `enum 名` の定義の行(前に同じ行の属性と可視性があってもよい)なら、その名前。
fn 型の定義の名前(行: &str) -> Option<String> {
    let 残り = 属性と可視性を読み飛ばす(行);
    let 後ろ = 残り.strip_prefix("struct ").or_else(|| 残り.strip_prefix("enum "))?;
    let 名前 = 先頭の識別子(後ろ.trim_start());
    (!名前.is_empty()).then_some(名前)
}
