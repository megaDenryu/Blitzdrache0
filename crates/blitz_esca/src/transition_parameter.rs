//! 遷移関数の実行に必要な引数を1つのオブジェクトへ束ねた遷移パラメータ(`MParameter`)。
//! 状態でもコマンドでも規則でもない、遷移を成立させる必須の入力(Esca第1段階では経過時間だけ)を、生の値の並びでなく名前付きのプロパティで渡すために置く。
//! `MParameter` の法則により `Option<T>` のフィールドを持たない(`cargo xtask conform` が検査する)。参照: `_doc/設計/設計オントロジー.md` 2.6節・3.5.1。

use blitz_design::{MParameter, M不変データ};

use crate::elapsed_time::経過時間;

/// 遷移関数へ渡す必須入力の束。Esca第1段階では経過時間だけを持つ。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 遷移パラメータ {
    pub 経過時間: 経過時間,
}

impl M不変データ for 遷移パラメータ {}
impl MParameter for 遷移パラメータ {}
