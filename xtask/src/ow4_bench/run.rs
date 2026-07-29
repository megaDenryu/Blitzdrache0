//! 1つの物量点を1回だけ走らせる工程。受け取るのは物量点とアセットルートと通し番号、返すのは
//! その実行で採れた計器一式である。リリース版の実行ファイルを直接起動し、その生存中にプロセスの
//! RAM・VRAMを周期採取しながら、終了時報告をファイルへ受けて読む。
//! 標準出力をファイルへ落とすのは、採取のあいだ誰も読まないパイプが埋まると子プロセスが書き込みで止まるためである。

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::memory_sampling::{メモリ最大, 実行しながら採取する, 採取条件};
use crate::report_parse::計数報告;
use crate::streaming_report::ストリーミング要約報告;

use super::measure::{CPU区間一式, GPU時間, Vulkan確保};
use super::{フレーム数, 上限バイト数, 先読み半径, 実行ファイル, 起動時シーン};

const 採取間隔: Duration = Duration::from_secs(1);
/// 打ち切りは異常の検出であって計測の期限ではないため、100倍の物量でも余る長さを取る。
const 制限時間: Duration = Duration::from_secs(300);

pub(super) struct 一回の実行 {
    pub(super) 区間: CPU区間一式,
    pub(super) gpu: GPU時間,
    pub(super) 確保: Vulkan確保,
    pub(super) 計数: 計数報告,
    pub(super) 要約: ストリーミング要約報告,
    /// GPUカウンターを1標本も引けなかった実行では専用VRAMが不在になる。
    pub(super) プロセス: メモリ最大,
}

pub(super) fn 走らせる(
    出力先: &Path, 名前: &str, アセットルート: &Path, シェーダー入口: &Path
) -> Result<一回の実行, String> {
    let 上限 = 上限バイト数.to_string();
    let 標準出力先 = PathBuf::from(出力先).join(format!("{名前}.log"));
    let 引数一覧 = 引数を作る(アセットルート, シェーダー入口, &上限);
    let 引数参照: Vec<&str> = 引数一覧.iter().map(String::as_str).collect();
    println!("[xtask] ow4-bench実行{名前}: {}", 引数参照.join(" "));
    let 条件 = 採取条件 {
        実行ファイル,
        引数一覧: &引数参照,
        採取間隔,
        制限時間,
        標準出力先: Some(&標準出力先),
    };
    let プロセス = 実行しながら採取する(&条件).ok_or_else(|| format!("実行{名前}が失敗したか標本を1つも採れなかった"))?;
    let 標準出力 = std::fs::read_to_string(&標準出力先).map_err(|誤り| format!("{}を読めない: {誤り}", 標準出力先.display()))?;
    読み取る(&標準出力, プロセス)
}

fn 読み取る(標準出力: &str, プロセス: メモリ最大) -> Result<一回の実行, String> {
    let 計数 = crate::report_parse::取り出す(標準出力)?;
    if 計数.validation件数 != 0 {
        return Err(format!("validationのエラー・警告が{}件あった", 計数.validation件数));
    }
    Ok(一回の実行 {
        区間: super::measure::cpu区間を取り出す(標準出力)?,
        gpu: super::measure::gpu時間を取り出す(標準出力)?,
        確保: super::measure::vulkan確保を取り出す(標準出力)?,
        計数,
        要約: crate::streaming_report::取り出す(標準出力)?,
        プロセス,
    })
}

fn 引数を作る(アセットルート: &Path, シェーダー入口: &Path, 上限: &str) -> Vec<String> {
    let 固定 = [
        "--scene",
        起動時シーン,
        "--streaming",
        "--streaming-preload-radius",
        先読み半径,
        "--instance-stream-route",
        "--benchmark-frames",
        フレーム数,
        "--report-streaming-summary",
        "--report-memory",
        "--report-draw-issue",
        "--report-instance-sections",
        "--report-gpu-times",
        "--report-frame-times",
        "--unlit",
        "--no-post",
        // 地形世界は空を持つ方針であるため、既存の計測条件を保つには空パスを明示的に外す。
        "--no-sky",
    ];
    let mut 引数一覧: Vec<String> = 固定.iter().map(|語| (*語).to_string()).collect();
    引数一覧.extend(["--asset-root".to_string(), アセットルート.display().to_string()]);
    引数一覧.extend(["--shader-source".to_string(), シェーダー入口.display().to_string()]);
    引数一覧.extend(["--streaming-ram-limit".to_string(), 上限.to_string()]);
    引数一覧.extend(["--streaming-vram-limit".to_string(), 上限.to_string()]);
    引数一覧
}
