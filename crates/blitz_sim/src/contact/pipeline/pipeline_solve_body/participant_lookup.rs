//! 剛体どうしの接触点集合の1回反復が読み書きする、台帳からの剛体の引きと動的剛体の予測の読み書き。
//! 親モジュール(`pipeline_solve_body`)から分けているのは、こちらが名前の付く入力と出力を持つ独立した
//! 工程であり、呼び出し元が中身(配列添字での探索)を知らずに呼べるためである。

use super::super::substep_predict::細分の動的剛体;
use crate::contact::pipeline::pipeline_error::接触の工程エラー;
use crate::rigid_body::{剛体, 剛体の識別子, 剛体エラー};

pub(super) fn 台帳から引く(剛体一覧: &[剛体], 識別子: 剛体の識別子) -> Result<&剛体, 接触の工程エラー> {
    Ok(剛体一覧.get(識別子.配列添字()).ok_or(剛体エラー::剛体が登録されていない { 識別子 })?)
}

// 起きている動的剛体だけがこの細分の予測を持つ。
pub(super) fn 予測を探す(
    動的剛体一覧: &[細分の動的剛体], 識別子: 剛体の識別子
) -> Option<crate::rigid_xpbd::予測の状態> {
    動的剛体一覧.iter().find(|剛体| 剛体.識別子 == 識別子).map(|剛体| 剛体.予測)
}

pub(super) fn 予測を書き戻す(
    動的剛体一覧: &mut [細分の動的剛体], 識別子: 剛体の識別子, 予測: Option<crate::rigid_xpbd::予測の状態>
) {
    let (Some(予測), Some(対象)) = (予測, 動的剛体一覧.iter_mut().find(|剛体| 剛体.識別子 == 識別子)) else {
        return;
    };
    対象.予測 = 予測;
}
