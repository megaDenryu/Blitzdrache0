//! フェンス待機後にGPU計測を読み、フレーム固有バッファを書き込む。

use super::super::cpu_timing::CPU区間時計;
use super::super::frame_progress::フレームスロット資源;
use super::super::レンダラー;
use crate::error::レンダラーエラー;
use crate::frame_input::フレーム描画入力;
use crate::view_input::描画視点;

pub(super) struct 描画準備 {
    pub(super) cpu区間時計: Option<CPU区間時計>,
    pub(super) スロット資源: フレームスロット資源,
}

pub(super) fn 実行する(
    レンダラー: &mut レンダラー,
    入力: &フレーム描画入力<'_>,
    視点: &描画視点,
    ビュー射影行列: &[[f32; 4]; 4],
) -> Result<描画準備, レンダラーエラー> {
    let mut cpu区間時計 = レンダラー.cpu区間計測.as_ref().map(|_| CPU区間時計::開始する());
    let スロット資源 = レンダラー.フレーム進行.現在のスロット資源();
    let フレーム添字 = スロット資源.スロット;
    // 安全性: フェンスは前フレームまたは初期シグナル状態のGPU完了を表す。
    unsafe { レンダラー.環境.device().wait_for_fences(&[スロット資源.フェンス], true, u64::MAX)? };
    if let Some(時計) = &mut cpu区間時計 {
        時計.フェンス待機を終了する();
    }
    if let Some(計測) = &mut レンダラー.gpu計測 {
        計測.読み取る(レンダラー.環境.device(), フレーム添字);
    }
    レンダラー.大気媒体の一致を確かめる(入力)?;
    レンダラー.ユニフォームを書き込む(フレーム添字, 入力, 視点, ビュー射影行列)?;
    レンダラー.スキン行列を書き込む(フレーム添字, 入力)?;
    レンダラー.布フレームを書き込む(フレーム添字, 入力)?;
    if let Some(時計) = &mut cpu区間時計 {
        時計.フレームデータ準備を終了する();
    }
    Ok(描画準備 {
        cpu区間時計, スロット資源
    })
}
