//! 布生成の単体テスト(判断52・55): 拘束本数の理論値一致・隣接本数(内部8・角3)。

#![allow(clippy::unwrap_used)]

use super::adjacency_entry::空き添字;
use super::generate::布を生成する;
use super::spec::布仕様;

fn 試験用仕様(一辺粒子数: u32) -> 布仕様 {
    布仕様::生成する(一辺粒子数, 1.0, 10.0, [0.0, 0.0, 0.0]).unwrap()
}

#[test]
fn 拘束本数が理論値と一致する() {
    let n: usize = 32;
    let 布 = 布を生成する(&試験用仕様(u32::try_from(n).unwrap())).unwrap();
    let 構造本数 = 2 * n * (n - 1);
    let せん断本数 = 2 * (n - 1) * (n - 1);
    assert_eq!(布.距離拘束一覧.len(), 構造本数 + せん断本数);
}

#[test]
fn 内部粒子の隣接本数は8で角は3になる() {
    let n: usize = 5;
    let 布 = 布を生成する(&試験用仕様(u32::try_from(n).unwrap())).unwrap();

    let 中央添字 = 2 * n + 2;
    let 内部本数 = 布.隣接拘束一覧[中央添字]
        .iter()
        .filter(|エントリ| エントリ.相手粒子添字 != 空き添字)
        .count();
    assert_eq!(内部本数, 8);

    let 角本数 = 布.隣接拘束一覧[0].iter().filter(|エントリ| エントリ.相手粒子添字 != 空き添字).count();
    assert_eq!(角本数, 3);
}

#[test]
fn 上端行の粒子添字は先頭一辺粒子数ぶんになる() {
    let n: u32 = 4;
    let 布 = 布を生成する(&試験用仕様(n)).unwrap();
    let 期待: Vec<u32> = (0..n).collect();
    assert_eq!(布.上端行の粒子添字一覧, 期待);
}

#[test]
fn 描画用インデックスは1セルあたり2三角形6添字になる() {
    let n: usize = 4;
    let 布 = 布を生成する(&試験用仕様(u32::try_from(n).unwrap())).unwrap();
    assert_eq!(布.描画用インデックス一覧.len(), (n - 1) * (n - 1) * 6);
}
