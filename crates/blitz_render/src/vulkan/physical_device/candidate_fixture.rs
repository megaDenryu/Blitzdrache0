//! 選定の検査が使う候補の組み立て。担うのは、物理デバイスに触れずに機能の有無だけを変えた候補を作ることである。
//! 検査そのものは持たない。

use super::candidate::選定候補;
use crate::vulkan::descriptor_indexing::ディスクリプタ索引機能;

pub(super) fn 候補(添字: usize, 機材名: &str, discreteか: bool, 索引対応: bool) -> 選定候補 {
    ブロック圧縮を選べる候補(添字, 機材名, discreteか, 索引対応, true)
}

pub(super) fn ブロック圧縮を選べる候補(
    添字: usize,
    機材名: &str,
    discreteか: bool,
    索引対応: bool,
    ブロック圧縮対応: bool,
) -> 選定候補 {
    機能を選べる候補(添字, 機材名, discreteか, 索引対応, ブロック圧縮対応, true)
}

pub(super) fn 機能を選べる候補(
    添字: usize,
    機材名: &str,
    discreteか: bool,
    索引対応: bool,
    ブロック圧縮対応: bool,
    立方体配列対応: bool,
) -> 選定候補 {
    let 機能 = ディスクリプタ索引機能::生成する(索引対応, 索引対応);
    選定候補::生成する(添字, 機材名.to_string(), discreteか, 機能, ブロック圧縮対応, 立方体配列対応)
}
