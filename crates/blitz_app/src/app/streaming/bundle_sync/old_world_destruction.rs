//! 目録差し替え前に解除予約した束と、現在の台帳へ解除を報告する束を識別する工程。

use blitz_engine::ストリーミング調停;
use blitz_render::描画束ID;

use crate::error::起動エラー;

use super::束idから座標を復元する;

pub(super) fn 差し替え前の実破棄を処理する(
    調停: &mut ストリーミング調停,
    実破棄済み束: &[描画束ID],
    差し替え前の実破棄待ち束: &mut Vec<描画束ID>,
) -> Result<(), 起動エラー> {
    for 束id in 実破棄済み束 {
        if let Some(位置) = 差し替え前の実破棄待ち束.iter().position(|候補| 候補 == 束id) {
            差し替え前の実破棄待ち束.swap_remove(位置);
        } else {
            調停.gpu資源の解除を報告する(束idから座標を復元する(*束id))?;
        }
    }
    Ok(())
}
