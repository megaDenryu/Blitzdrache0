//! 複数候補での選び分けの検査。物理デバイスに触れずに候補を組み立て、索引の機能またはテクスチャのブロック圧縮を
//! 欠く候補を飛ばすこと、discreteを優先すること、1台も残らないときにどちらの条件で落ちたのかを報告することを見る。

use super::candidate::選定候補;
use super::choose::選ぶ;
use crate::error::{ディスクリプタ索引機能項目, デバイス要件エラー};
use crate::vulkan::descriptor_indexing::ディスクリプタ索引機能;

fn 候補(添字: usize, 機材名: &str, discreteか: bool, 索引対応: bool) -> 選定候補 {
    ブロック圧縮を選べる候補(添字, 機材名, discreteか, 索引対応, true)
}

fn ブロック圧縮を選べる候補(
    添字: usize, 機材名: &str, discreteか: bool, 索引対応: bool, ブロック圧縮対応: bool
) -> 選定候補 {
    let 機能 = ディスクリプタ索引機能::生成する(索引対応, 索引対応);
    選定候補::生成する(添字, 機材名.to_string(), discreteか, 機能, ブロック圧縮対応)
}

fn 不足内訳を取り出す(候補一覧: &[選定候補]) -> Vec<(String, Vec<ディスクリプタ索引機能項目>)> {
    match 選ぶ(候補一覧) {
        Ok(添字) => panic!("失敗するはずの候補一覧で添字{添字}が選ばれた"),
        Err(デバイス要件エラー::ディスクリプタ索引機能不足(内訳一覧)) => 内訳一覧
            .iter()
            .map(|内訳| (内訳.機材名().to_string(), 内訳.不足一覧().to_vec()))
            .collect(),
        Err(誤り) => panic!("候補別の不足以外のエラーが返った: {誤り}"),
    }
}

#[test]
fn 先頭候補が索引の機能を欠くなら対応する次の候補を選ぶ() {
    let 候補一覧 = [候補(0, "統合GPU", false, false), 候補(1, "別の統合GPU", false, true)];
    assert_eq!(選ぶ(&候補一覧).ok(), Some(1));
}

#[test]
fn 索引に対応する候補の中でdiscreteを優先する() {
    let 候補一覧 = [候補(0, "統合GPU", false, true), 候補(1, "discrete GPU", true, true)];
    assert_eq!(選ぶ(&候補一覧).ok(), Some(1));
}

/// discreteが索引の機能を欠く機材でも、対応する統合GPUがあるなら起動できなければならない。
#[test]
fn discreteが索引の機能を欠くなら対応する非discreteの候補を選ぶ() {
    let 候補一覧 = [候補(0, "discrete GPU", true, false), 候補(1, "統合GPU", false, true)];
    assert_eq!(選ぶ(&候補一覧).ok(), Some(1));
}

#[test]
fn 対応候補が非discreteだけなら先頭を選ぶ() {
    let 候補一覧 = [候補(0, "統合GPU", false, true), 候補(1, "別の統合GPU", false, true)];
    assert_eq!(選ぶ(&候補一覧).ok(), Some(0));
}

#[test]
fn ブロック圧縮を欠く候補を飛ばして対応する候補を選ぶ() {
    let 候補一覧 = [
        ブロック圧縮を選べる候補(0, "discrete GPU", true, true, false),
        候補(1, "統合GPU", false, true),
    ];
    assert_eq!(選ぶ(&候補一覧).ok(), Some(1));
}

#[test]
fn 索引に対応する候補が全てブロック圧縮を欠くと機材名を並べて報告する() {
    let 候補一覧 = [ブロック圧縮を選べる候補(0, "discrete GPU", true, true, false)];
    let Err(デバイス要件エラー::テクスチャのブロック圧縮非対応(機材名一覧)) = 選ぶ(&候補一覧) else {
        panic!("ブロック圧縮の非対応以外の結果が返った");
    };
    assert_eq!(機材名一覧, vec!["discrete GPU".to_string()]);
}

#[test]
fn 全候補が索引の機能を欠くと候補別の不足を報告する() {
    let 欠ける候補 = 選定候補::生成する(1, "統合GPU".to_string(), false, ディスクリプタ索引機能::生成する(false, true), true);
    let 候補一覧 = [候補(0, "discrete GPU", true, false), 欠ける候補];
    let 内訳一覧 = 不足内訳を取り出す(&候補一覧);
    assert_eq!(内訳一覧.len(), 2);
    assert_eq!(内訳一覧[0].0, "discrete GPU");
    assert_eq!(内訳一覧[0].1, ディスクリプタ索引機能項目::全項目.to_vec());
    assert_eq!(内訳一覧[1].0, "統合GPU");
    assert_eq!(内訳一覧[1].1, vec![ディスクリプタ索引機能項目::非一様な添字での画像参照]);
}

#[test]
fn 基礎要件を満たす候補が無ければ適合物理デバイスなしになる() {
    assert!(matches!(選ぶ(&[]), Err(デバイス要件エラー::適合物理デバイスなし)));
}

#[test]
fn 全候補が欠けたときのメッセージに機材名と機能名が並ぶ() {
    let 文言 = 選ぶ(&[候補(0, "discrete GPU", true, false)])
        .err()
        .map(|誤り| 誤り.to_string())
        .unwrap_or_default();
    assert!(文言.contains("discrete GPU"), "実際のメッセージ: {文言}");
    assert!(文言.contains("shaderSampledImageArrayNonUniformIndexing"), "実際のメッセージ: {文言}");
}
