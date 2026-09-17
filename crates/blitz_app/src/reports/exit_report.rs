//! 終了時の報告。受け取るのは`アプリ`が各束から1回だけ集めた終了時の観測一式、返すものは無い。担当するのは、
//! 報告要求に応じてどの報告をどの順で出すかだけであり、行の整形は`reports`配下の各報告が持つ。
//!
//! レンダラーが据わらなかった実行(1フレームも描いていない)では、レンダラーの内部を読む報告は「取得できない」旨の1行を出す。
//! 描画発行の報告と、大気と間接照明の焼き上げの報告は行が多いため`draw_issue_section`と`bake_passes_section`が持つ。

mod bake_passes_section;
mod draw_issue_section;

use blitz_render::レンダラー;

use crate::app::終了時の観測一式;
use crate::reports::{display_timing, draw_issue, game, gpu_frame_samples, gpu_time_table, streaming_summary, sun_angle};

pub(crate) fn 終了時報告を出す(観測: &終了時の観測一式<'_>) {
    let 要求 = 観測.報告要求;
    if let Some(要約) = &観測.ゲームの進行 {
        game::ゲームの進行を表示する(要約, 観測.カメラ大域位置);
    }
    if 要求.gpu時間 {
        gpu_time_table::パス別gpu時間の表を表示する(&観測.レンダラー.map(レンダラー::パス別gpu時間を取得する).unwrap_or_default());
    }
    if 要求.gpu時間のフレーム別生値 {
        gpu_frame_samples::パス別gpu時間のフレーム別生値を表示する(観測.レンダラー.map_or(&[], レンダラー::パス別gpu時間のフレーム別標本を取得する));
    }
    if 要求.大気のベイク済み画像生成パス数 {
        bake_passes_section::大気のベイク済み画像の生成パス数の節を出す(観測);
    }
    if 要求.gpuメモリ {
        match 観測.レンダラー {
            Some(レンダラー) => super::gpuメモリ統計を表示する(&レンダラー.gpuメモリ統計を取得する()),
            None => println!("Vulkanメモリ確保: レンダラーが生成されなかったため取得できない"),
        }
    }
    if 要求.描画発行 {
        draw_issue_section::描画発行の内訳の節を出す(観測);
    }
    if 要求.キャスター距離分布 {
        draw_issue::キャスター距離分布を表示する(&観測.描画束.選別の計器.距離分布);
    }
    if 要求.太陽角度 {
        sun_angle::太陽の角度を表示する(観測.天空.天空状態);
    }
    let cpu区間時間: &[blitz_render::CPU区間時間] = 観測.レンダラー.map_or(&[], レンダラー::cpu区間時間一覧を取得する);
    if 観測.検収の観測.フレーム時間報告が必要か {
        match &観測.検収の観測.フレーム時間統計 {
            Some(統計) => super::フレーム時間統計を表示する(統計),
            None => println!("CPU側フレーム間隔: 計測できなかった(ウォームアップ後のフレームがない)"),
        }
        super::レンダラーcpu区間を表示する(cpu区間時間);
    }
    if 観測.検収の観測.インスタンス区間報告が必要か {
        if !観測.検収の観測.フレーム時間報告が必要か {
            super::レンダラーcpu区間を表示する(cpu区間時間);
        }
        super::可視個体の選別の区間を表示する(観測.検収の観測.選別の区間統計.as_ref());
    }
    if 要求.ストリーミング要約 {
        match &観測.ストリーミング要約 {
            Some(要約) => streaming_summary::ストリーミング要約を表示する(要約),
            None => println!("ストリーミング要約: --streamingが指定されていないため計測していない"),
        }
    }
    if 要求.実表示時間 {
        display_timing::実表示間隔を表示する(観測.レンダラー.map(レンダラー::実表示計測状況を取得する), 観測.レンダラー.map_or(&[], レンダラー::実表示観測一覧を取得する));
    }
}
