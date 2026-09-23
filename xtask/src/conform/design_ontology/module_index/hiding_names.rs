//! 索引に入れない位置(局所の位置とマクロの呼び出しの中。`module_path/enclosing_module.rs`)に書いた `use` と項目が持ち込みうる名前の控え。
//! モジュールの索引は、それらの `use` と項目をモジュールの取り込みにも宣言にも数えない。局所の `use` と項目はその波括弧の内側でしか見えず、マクロの呼び出しの中の項目は展開した後の名前を検査器が知らないためである。
//! ただしそれらは、実装が書いた名前を隠しうる。名前を解く工程は、隠しうるかをこの控えに問い、隠しうる名前を書いた実装を黙って通さず違反の側へ倒す。
//! 控えは、どの波括弧の内側かを区別せずに名前を集める。隠しうる名前を広く見る側へ倒すためである。

use std::collections::HashSet;

use super::super::use_resolution::取り込みの項目;
use super::file_lexical_position::項目の宣言の名前;
use super::type_alias_table::型の別名の宣言;

#[derive(Default)]
pub struct 隠しうる名前の控え {
    宣言した名前一覧: HashSet<String>,       // `struct`・`enum`・`union`・`type`・`trait`・`mod` の名前
    取り込みの項目一覧: Vec<取り込みの項目>, // `use` の項目
    型の別名一覧: Vec<(String, String)>,     // `type 別名 = 右辺;` の別名と右辺の最後の名前の組
}

impl 隠しうる名前の控え {
    /// 1行が項目の宣言か `type` の別名なら、その名前を控える。
    pub fn 行を足す(&mut self, 行: &str) {
        self.宣言した名前一覧.extend(項目の宣言の名前(行));
        self.型の別名一覧.extend(型の別名の宣言(行));
    }

    pub fn 取り込みを足す(&mut self, 項目: 取り込みの項目) {
        self.取り込みの項目一覧.push(項目);
    }

    /// 控えた項目か `use` がその名前を持ち込みうるか。glob の取り込みがあれば、どの名前も持ち込みうる。
    pub fn 名前を隠しうるか(&self, 名前: &str) -> bool {
        self.宣言した名前一覧.contains(名前) || self.取り込みの項目一覧.iter().any(|項目| 項目.名乗る名前() == 名前 || 項目.全部を取り込むか())
    }

    /// 控えた `use 元 as 別名` か `type 別名 = 元;` が、その別名で型名を指しうるか。
    pub fn 別名が型名を指しうるか(&self, 別名: &str, 型名: &str) -> bool {
        self.取り込みの項目一覧.iter().any(|項目| 項目.別名.as_deref() == Some(別名) && 項目.元の名前() == 型名) || self.型の別名一覧.iter().any(|(左辺, 右辺)| 左辺 == 別名 && 右辺 == 型名)
    }
}
