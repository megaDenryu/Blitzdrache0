//! スモーク計画が要求したシェーダーまたはアセットの書き換えを適用する。

use super::super::アプリ;
use crate::error::起動エラー;
use crate::smoke::スモークアクション;

pub(super) fn 適用する(アプリ: &アプリ, アクション: スモークアクション) -> Result<(), 起動エラー> {
    let Some(スモーク実行) = &アプリ.スモーク実行 else {
        return Ok(());
    };
    スモーク実行.書き換えを適用する(アクション)
}
