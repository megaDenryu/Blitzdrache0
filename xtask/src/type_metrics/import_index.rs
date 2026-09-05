//! 1つのファイルが`use`で持ち込んだ名前から、その経路を引く索引。
//!
//! implブロックが`impl 設定 {`と短い名前だけを書いたとき、その`設定`がどこの`設定`かを言える材料は
//! そのファイルの`use`宣言だけである。同じ名前を持ち込む宣言が2件あれば両方を返し、絞り込みは
//! 引き当てる側が定義の候補と突き合わせて行う。

use std::collections::BTreeMap;

use super::import_line::取り込みの宣言一覧;
use super::import_tree::取り込みの項を経路へ展開する;
use super::type_path::自己型の経路;

pub struct 取り込みの索引 {
    型名ごとの経路一覧: BTreeMap<String, Vec<自己型の経路>>,
}

impl 取り込みの索引 {
    pub fn ファイルの内容から生成する(内容: &str) -> Self {
        let mut 型名ごとの経路一覧: BTreeMap<String, Vec<自己型の経路>> = BTreeMap::new();
        for 綴り in 取り込みの宣言一覧(内容).iter().flat_map(|宣言| 取り込みの項を経路へ展開する("", 宣言)) {
            let 経路 = 自己型の経路::綴りから生成する(&綴り);
            型名ごとの経路一覧.entry(経路.型名().to_string()).or_default().push(経路);
        }
        Self { 型名ごとの経路一覧 }
    }

    pub fn 型名から経路一覧を引く(&self, 型名: &str) -> &[自己型の経路] {
        self.型名ごとの経路一覧.get(型名).map_or(&[][..], Vec::as_slice)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn 持ち込んだ名前から経路を引ける() {
        let 索引 = 取り込みの索引::ファイルの内容から生成する("use super::sweep_solver::求解;\nuse std::path::Path;\n");
        let 経路一覧 = 索引.型名から経路一覧を引く("求解");
        assert_eq!(経路一覧.len(), 1);
        assert!(
            経路一覧[0]
                .定義ファイルの候補一覧(Path::new("a/src/triangle/dispatch.rs"))
                .contains(&"a/src/triangle/sweep_solver.rs".to_string())
        );
    }

    #[test]
    fn 持ち込んでいない名前は空を返す() {
        let 索引 = 取り込みの索引::ファイルの内容から生成する("use super::求解;\n");
        assert!(索引.型名から経路一覧を引く("設定").is_empty());
    }
}
