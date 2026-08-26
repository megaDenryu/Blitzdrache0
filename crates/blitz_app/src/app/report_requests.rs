//! 終了時にどの報告を出すかの要求。担当するのは「起動指定のうち報告の有無だけを1つの型で保つ」ことである。
//!
//! 真偽値をアプリ直下へ並べないのは、報告の種類が増えるたびにアプリのフィールドが増え、どれが状態でどれが
//! 起動時の選択かの区別が薄れるためである(描画段階資源が段階の資源を1つに束ねるのと同じ理由)。
//! 「求められたか」を答えるのは`queries/report_request`であり、ここは値を保つだけである。

use crate::cli::起動設定;

/// 終了時に出す報告の要求一式。フィールドはどれも対応するCLI引数の指定有無を保つだけの真偽値である。
///
/// - `実表示時間`: `--report-display-timing`の計測は提示待機で描画ループを止めるため、フレーム時間報告とは独立に切り替える。
pub(crate) struct 報告要求 {
    pub(super) gpu時間: bool,                        // --report-gpu-times。パス別GPU時間の移動平均を出す
    pub(super) gpu時間のフレーム別生値: bool,        // --report-gpu-frame-times。パス別GPU時間の生値をフレーム別に全件出す
    pub(super) 大気のベイク済み画像生成パス数: bool, // --report-atmosphere-passes。フレームごとの生成パス本数を出す
    pub(super) gpuメモリ: bool,                      // --report-memory。Vulkan専用メモリの確保数と用途別量を出す
    pub(super) 描画発行: bool,                       // --report-draw-issue。最終フレームのパス別描画発行数を出す
    pub(super) 太陽角度: bool,                       // --report-sun-angle。その実行が使った太陽の高度と方位を出す
    pub(super) キャスター距離分布: bool,             // --report-caster-distance。最終フレームのキャスター候補の距離帯別分布を出す
    pub(super) 実表示時間: bool,                     // --report-display-timing。提示待機で測った実表示間隔を出す
    pub(super) ストリーミング要約: bool,             // --report-streaming-summary。ストリーミング無効時も指定の有無を保つ
}

impl 報告要求 {
    pub(super) fn 起動設定から作る(設定: &起動設定) -> Self {
        Self {
            gpu時間: 設定.gpu時間報告,
            gpu時間のフレーム別生値: 設定.gpu時間のフレーム別生値報告,
            大気のベイク済み画像生成パス数: 設定.大気のベイク済み画像パス数報告,
            gpuメモリ: 設定.gpuメモリ報告,
            描画発行: 設定.描画発行報告,
            太陽角度: 設定.太陽角度報告,
            キャスター距離分布: 設定.キャスター距離分布報告,
            実表示時間: 設定.実表示時間報告,
            ストリーミング要約: 設定.ストリーミング.要約を報告する,
        }
    }
}
