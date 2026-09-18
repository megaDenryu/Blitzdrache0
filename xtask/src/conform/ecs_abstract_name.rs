//! 個体群の基盤(`blitz_ecs`)の中に、一般的なECSの抽象名が現れていないかの検査。
//! 設計正本の判断1は、クレート名にだけ設計方式の略称を許し、中の型名・メソッド名・フィールド名をすべて
//! 日本語に保つことを求める(参照: `_doc/設計/ゲーム世界の個体群の基盤.md`「判断1」)。
//! クレート名まで見るのは、同じ抽象名を正当に使う他のクレートを巻き添えにしないためである。
//! 注意: 検出する綴りをこのファイルへ連続して書くと自分自身が検出対象になりうるため、分割リテラルの連結で回避する。

use std::path::Path;

use super::violation::違反;

const 対象クレート: &str = "blitz_ecs";
const 抽象名一覧: [&str; 4] = [concat!("Wor", "ld"), concat!("Que", "ry"), concat!("Compo", "nent"), concat!("Enti", "ty")];

fn 個体群の基盤のクレートの中のファイルか(パス: &Path) -> bool {
    let 部品一覧: Vec<&std::ffi::OsStr> = パス.components().map(|部品| 部品.as_os_str()).collect();
    部品一覧.iter().any(|部品| *部品 == 対象クレート) && 部品一覧.iter().any(|部品| *部品 == "src")
}

pub fn 個体群の基盤の抽象名を検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    if !個体群の基盤のクレートの中のファイルか(パス) {
        return Vec::new();
    }
    内容
        .lines()
        .enumerate()
        .filter_map(|(添字, 行)| 抽象名一覧.iter().find(|抽象名| 行.contains(*抽象名)).map(|抽象名| (添字, 抽象名)))
        .map(|(添字, 抽象名)| 違反::行単位(パス.to_path_buf(), 添字 + 1, format!("個体群の基盤で一般的なECSの抽象名({抽象名})を使っている(日本語の名前へ替える)")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 個体群の基盤の抽象名だけを違反にする() {
        let 原文 = concat!("pub struct Wor", "ld;\n");
        assert_eq!(個体群の基盤の抽象名を検査する(Path::new("crates/blitz_ecs/src/lib.rs"), 原文).len(), 1);
        assert!(個体群の基盤の抽象名を検査する(Path::new("crates/blitz_app/src/world_execution/entity_id.rs"), 原文).is_empty());
    }

    #[test]
    fn コメントの中の抽象名も違反にする() {
        let 原文 = concat!("// Compo", "nent を持つ\nlet a = 1;\n");
        assert_eq!(個体群の基盤の抽象名を検査する(Path::new("crates/blitz_ecs/src/storage.rs"), 原文).len(), 1);
    }

    #[test]
    fn 日本語の名前は違反にしない() {
        let 原文 = "pub struct ゲーム世界の個体群;\n";
        assert!(個体群の基盤の抽象名を検査する(Path::new("crates/blitz_ecs/src/lib.rs"), 原文).is_empty());
    }
}
