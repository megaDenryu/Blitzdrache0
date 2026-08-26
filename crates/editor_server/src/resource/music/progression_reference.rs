//! コード進行の参照の型契約。
//!
//! 既定の進行と独自の進行を同じ名前空間へ混ぜず判別共用体で分けるのは、独自の進行に既定と同じ名前を
//! 付けたときにどちらを指すか決まらなくなるためである。実在しない参照を無言で既定へ落とさず明示の
//! 失敗にする規律もこの判別が支える(参照: `_doc/設計/楽曲エディター.md`「判断4」)。解決は`進行の名簿`が持つ。

use serde::{Deserialize, Serialize};

/// コード進行参照とは、トラックまたはパターンがどのコード進行に従うかを1つに特定する判別のことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(tag = "種類")]
pub enum コード進行参照 {
    既定の進行 { 識別子: String },
    独自の進行 { 名前: String },
}
