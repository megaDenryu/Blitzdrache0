//! 1つの物量点を1回だけ走らせる工程。受け取るのは物量点とアセットルートと通し番号と計測条件、返すのはその実行で採れた計器一式である。リリース版の実行ファイルを直接起動し、その生存中にプロセスのRAM・VRAMを周期採取しながら、終了時報告をファイルへ受けて読む。
//! 標準出力をファイルへ落とすのは、採取のあいだ誰も読まないパイプが埋まると子プロセスが書き込みで止まるためである。
//! 時間再構成は`--no-taa`で外す。段3b以前に採った値と並べて読むため、パスを1本足した条件へ計測窓を変えない。

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::acceptance::{判定の名前, 検収の実行名, 終了時報告};
use crate::memory_sampling::{メモリ最大, 実行しながら採取する, 採取条件};
use crate::report_parse::計数報告;
use crate::streaming_report::ストリーミング要約報告;

use super::condition::計測条件;
use super::error::物量計測エラー;
use super::gpu_table::GPU時間;
use super::measure::{CPU区間一式, Vulkan確保};
use launch_arguments::{上限の綴り, 引数を作る};

mod launch_arguments;

/// 計測の本体はリリースビルドで走るが、報告の件数が零でない実行は表へ載せない。
const 検証層のエラーと警告: 判定の名前 = 判定の名前::定数から生成する("検証層のエラーと警告");

const 採取間隔: Duration = Duration::from_secs(1);
/// 打ち切りは異常の検出であって計測の期限ではないため、100倍の物量でも余る長さを取る。
const 制限時間: Duration = Duration::from_secs(300);

pub(super) struct 一回の実行 {
    pub(super) 区間: CPU区間一式,
    pub(super) gpu: GPU時間,
    pub(super) 確保: Vulkan確保,
    pub(super) 計数: 計数報告,
    /// 距離区分ごとの、シャドウマップの1テクセルが世界で覆う長さ。添字が距離区分番号である。
    pub(super) 世界メートル毎テクセル: Vec<f64>,
    pub(super) 要約: ストリーミング要約報告,
    /// GPUカウンターを1標本も引けなかった実行では専用VRAMが不在になる。
    pub(super) プロセス: メモリ最大,
}

pub(super) fn 走らせる(
    出力先: &Path,
    名前: &str,
    アセットルート: &Path,
    シェーダー入口: &Path,
    条件: &計測条件,
) -> Result<一回の実行, 物量計測エラー> {
    let 上限 = 上限の綴り();
    let 標準出力先 = PathBuf::from(出力先).join(format!("{名前}.log"));
    let 引数一覧 = 引数を作る(アセットルート, シェーダー入口, &上限, 条件);
    let 引数参照: Vec<&str> = 引数一覧.iter().map(String::as_str).collect();
    println!("[xtask] ow4-bench実行{名前}: {}", 引数参照.join(" "));
    let 条件 = 採取条件 {
        起こし方: crate::acceptance::アプリの起こし方::構築済みのリリース版を直に起動する,
        引数一覧: &引数参照,
        採取間隔,
        制限時間,
        標準出力先: Some(&標準出力先),
    };
    let プロセス = 実行しながら採取する(&条件)
        .ok_or_else(|| 物量計測エラー::実行が標本を1つも採れなかった { 名前: 名前.to_string() })?;
    let 標準出力 = std::fs::read_to_string(&標準出力先).map_err(|誤り| 物量計測エラー::実行の標準出力を読めなかった {
        パス: 標準出力先.clone(),
        誤り,
    })?;
    let 報告 = 終了時報告::取り込む(&検収の実行名::生成する(名前)?, 標準出力, String::new());
    読み取る(&報告, プロセス)
}

/// 注意: ファイルへ落とした標準出力を終了時報告として包み直す。この実行は採取のあいだ標準出力をファイルへ流すため、
/// 起動のセッションからは報告が返らない。
fn 読み取る(報告: &終了時報告, プロセス: メモリ最大) -> Result<一回の実行, 物量計測エラー> {
    let 計数 = crate::report_parse::取り出す(報告)?;
    検証層のエラーと警告.零件であることを課す(計数.validation件数)?;
    Ok(一回の実行 {
        区間: super::measure::cpu区間を取り出す(報告)?,
        gpu: super::gpu_table::取り出す(報告)?,
        確保: super::measure::vulkan確保を取り出す(報告)?,
        計数,
        世界メートル毎テクセル: super::measure::世界メートル毎テクセルを取り出す(報告)?,
        要約: crate::streaming_report::取り出す(報告)?,
        プロセス,
    })
}
