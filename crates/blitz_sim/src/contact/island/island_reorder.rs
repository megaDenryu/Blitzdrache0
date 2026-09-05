//! 接触拘束のバッチを接触島順かつ鍵昇順へ並べ替える処理(判断17)。
//! 島の中では剛体と静的世界の接触拘束を鍵昇順で先に、剛体どうしの接触拘束を鍵昇順で次に解く。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断17: 接触島は動的剛体の連結成分であり、島の中の反復の順序は鍵の辞書式昇順である」

use super::super::body_body_contact::剛体と剛体の接触拘束;
use super::super::body_static_contact::剛体と静的世界の接触拘束;
use super::super::manifold_range::接触点集合の占める範囲;
use super::island_range::島の拘束の添字区間;

pub(super) fn 静的世界の接触を島順に並べ直す(
    拘束一覧: &[剛体と静的世界の接触拘束],
    範囲一覧: &[接触点集合の占める範囲],
    集合ごとの島番号: &[Option<usize>],
    島の数: usize,
) -> (
    Vec<剛体と静的世界の接触拘束>,
    Vec<接触点集合の占める範囲>,
    Vec<島の拘束の添字区間>,
    Vec<島の拘束の添字区間>,
) {
    let mut 順序: Vec<usize> = (0..範囲一覧.len()).collect();
    順序.sort_unstable_by_key(|&添字| {
        let 島 = 集合ごとの島番号.get(添字).copied().flatten().unwrap_or(usize::MAX);
        let 範囲 = 範囲一覧[添字];
        let 鍵 = 拘束一覧.get(範囲.開始の添字()).map(剛体と静的世界の接触拘束::履歴の鍵);
        (島, 鍵)
    });
    let mut 新しい拘束 = Vec::with_capacity(拘束一覧.len());
    let mut 新しい範囲 = Vec::with_capacity(範囲一覧.len());
    let mut 拘束の区間一覧 = vec![島の拘束の添字区間::空(); 島の数];
    let mut 集合の区間一覧 = vec![島の拘束の添字区間::空(); 島の数];
    for &添字 in &順序 {
        let Some(島) = 集合ごとの島番号.get(添字).copied().flatten() else {
            continue;
        };
        if 島 >= 島の数 {
            continue;
        }
        let 範囲 = 範囲一覧[添字];
        let 拘束の開始 = 新しい拘束.len();
        let 集合の開始 = 新しい範囲.len();
        let 切り出し = &拘束一覧[範囲.開始の添字()..範囲.終わりの添字()];
        新しい拘束.extend_from_slice(切り出し);
        let 拘束の終了 = 新しい拘束.len();
        新しい範囲.push(接触点集合の占める範囲::生成する(拘束の開始, 範囲.接触点の数()));
        let 集合の終了 = 新しい範囲.len();
        更新する区間(&mut 拘束の区間一覧[島], 拘束の開始, 拘束の終了);
        更新する区間(&mut 集合の区間一覧[島], 集合の開始, 集合の終了);
    }
    (新しい拘束, 新しい範囲, 拘束の区間一覧, 集合の区間一覧)
}

pub(super) fn 剛体どうしの接触を島順に並べ直す(
    拘束一覧: &[剛体と剛体の接触拘束],
    範囲一覧: &[接触点集合の占める範囲],
    集合ごとの島番号: &[Option<usize>],
    島の数: usize,
) -> (
    Vec<剛体と剛体の接触拘束>,
    Vec<接触点集合の占める範囲>,
    Vec<島の拘束の添字区間>,
    Vec<島の拘束の添字区間>,
) {
    let mut 順序: Vec<usize> = (0..範囲一覧.len()).collect();
    順序.sort_unstable_by_key(|&添字| {
        let 島 = 集合ごとの島番号.get(添字).copied().flatten().unwrap_or(usize::MAX);
        let 範囲 = 範囲一覧[添字];
        let 鍵 = 拘束一覧.get(範囲.開始の添字()).map(剛体と剛体の接触拘束::履歴の鍵);
        (島, 鍵)
    });
    let mut 新しい拘束 = Vec::with_capacity(拘束一覧.len());
    let mut 新しい範囲 = Vec::with_capacity(範囲一覧.len());
    let mut 拘束の区間一覧 = vec![島の拘束の添字区間::空(); 島の数];
    let mut 集合の区間一覧 = vec![島の拘束の添字区間::空(); 島の数];
    for &添字 in &順序 {
        let Some(島) = 集合ごとの島番号.get(添字).copied().flatten() else {
            continue;
        };
        if 島 >= 島の数 {
            continue;
        }
        let 範囲 = 範囲一覧[添字];
        let 拘束の開始 = 新しい拘束.len();
        let 集合の開始 = 新しい範囲.len();
        let 切り出し = &拘束一覧[範囲.開始の添字()..範囲.終わりの添字()];
        新しい拘束.extend_from_slice(切り出し);
        let 拘束の終了 = 新しい拘束.len();
        新しい範囲.push(接触点集合の占める範囲::生成する(拘束の開始, 範囲.接触点の数()));
        let 集合の終了 = 新しい範囲.len();
        更新する区間(&mut 拘束の区間一覧[島], 拘束の開始, 拘束の終了);
        更新する区間(&mut 集合の区間一覧[島], 集合の開始, 集合の終了);
    }
    (新しい拘束, 新しい範囲, 拘束の区間一覧, 集合の区間一覧)
}

fn 更新する区間(区間: &mut 島の拘束の添字区間, 開始: usize, 終了: usize) {
    if 区間.空か() {
        *区間 = 島の拘束の添字区間::生成する(開始, 終了);
    } else {
        *区間 = 島の拘束の添字区間::生成する(区間.開始(), 終了);
    }
}
