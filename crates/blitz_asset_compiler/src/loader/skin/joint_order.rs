//! ジョイントのトポロジカル順序を求める: 親の添字が子の添字より必ず小さくなるよう
//! 並べ替える(判断42の不変条件)。

use std::collections::HashMap;

/// 旧添字(スキンの`joints()`が返す順序)⇔新添字(トポロジカル順)の対応。
/// `旧親添字一覧[旧添字]`はその添字の親を旧添字空間で表したもの。
pub(super) struct 並べ替え {
    pub(super) 旧から新: Vec<usize>,
    pub(super) 新から旧: Vec<usize>,
    pub(super) 旧親添字一覧: Vec<Option<usize>>,
}

/// 各ノードの親ノードのグローバル添字を参照する表(ルートノードは含まれない)。
pub(super) fn 親ノード添字表を作る(文書: &gltf::Document) -> HashMap<usize, usize> {
    let mut 表 = HashMap::new();
    for ノード in 文書.nodes() {
        for 子 in ノード.children() {
            表.insert(子.index(), ノード.index());
        }
    }
    表
}

/// `旧グローバル一覧[旧添字]`が各ジョイントのグローバルノード添字。
pub(super) fn トポロジカル順を求める(旧グローバル一覧: &[usize], 親ノード添字表: &HashMap<usize, usize>) -> 並べ替え {
    let グローバルから旧添字: HashMap<usize, usize> = 旧グローバル一覧
        .iter()
        .enumerate()
        .map(|(旧添字, &グローバル)| (グローバル, 旧添字))
        .collect();

    let 旧親添字一覧: Vec<Option<usize>> = 旧グローバル一覧
        .iter()
        .map(|グローバル| {
            親ノード添字表
                .get(グローバル)
                .and_then(|親グローバル| グローバルから旧添字.get(親グローバル).copied())
        })
        .collect();

    let ジョイント数 = 旧グローバル一覧.len();
    let mut 旧から新 = vec![usize::MAX; ジョイント数];
    let mut 新から旧 = Vec::with_capacity(ジョイント数);
    for 開始 in 0..ジョイント数 {
        位置を確定する(開始, &旧親添字一覧, &mut 旧から新, &mut 新から旧);
    }

    並べ替え {
        旧から新,
        新から旧,
        旧親添字一覧,
    }
}

/// 親が先に確定するよう再帰的に位置を割り当てる。
fn 位置を確定する(旧添字: usize, 旧親添字一覧: &[Option<usize>], 旧から新: &mut Vec<usize>, 新から旧: &mut Vec<usize>) {
    if 旧から新[旧添字] != usize::MAX {
        return;
    }
    if let Some(親) = 旧親添字一覧[旧添字] {
        位置を確定する(親, 旧親添字一覧, 旧から新, 新から旧);
    }
    let 新添字 = 新から旧.len();
    新から旧.push(旧添字);
    旧から新[旧添字] = 新添字;
}
