//! 遠方環境の更新を間引く区間の境界を跨ぐ瞬間に絵へ出る段差を、一日ぶん全部実測する入口。
//! 目視見本の庭を、境界を跨いだ時刻へ固定したまま、遠方環境を焼く太陽天頂区間だけ下側と上側へ
//! 差し替えた対の圧縮前HDRで撮り、固定領域(地面の拡散と粗さ水準別の球)ごとに差を集計する。
//!
//! 判定値を1つも持たないのは、上限の制定がこの実測を読んでから行われるためである(ヒストグラム範囲の制定と同じ様式)。
//! 機械が見るのはvalidationの指摘0件・報告する値がすべて有限であること・代表点のA/B/A対照の3つである。
//!
//! 実時刻を境界の前後へ動かさないのは、直接光と空背景の連続変化が差へ混ざると間接照明の段差だけを測れなくなるためである。
//! 跨ぎの一覧は`--report-sun-zenith-crossings`が出す本番の導出から読む。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「間引きの実装設計」

mod control;
mod crossing;
mod difference;
mod hdr_image;
mod judgment;
mod measure;
mod picture;
mod record;
mod run;
mod scan;
mod summary;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::acceptance::{検収の実行名, 読み戻しの書き出し先};
use crate::release_build::計測の生値のファイル;

use crate::sample_world_region::固定領域一覧を作る;

const 出力ディレクトリ: &str = "target/ibl_step";
/// 領域マスクを撮るときのダンプ名。パスはASCIIで保つ。
const 領域マスクのファイル名: &str = "region_mask";
/// 段差を測る撮影のダンプのベース名。跨ぎの識別と向きと側が後ろへ付く。
const 撮影のベース名: &str = "shot";
/// 代表点の絵のダンプのベース名。段差を測る撮影と分けるのは、走査の生の画像を束ごとに捨てる一方、代表点の対は目視の材料として残すためである。
const 絵のベース名: &str = "picture";

pub fn 実行する() -> ExitCode {
    match 計測する() {
        Ok(要約) => {
            println!("[xtask] ibl-step成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] ibl-step失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 計測する() -> Result<String, String> {
    crate::visual_sample_world::用意する()?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let 領域マスクの実行名 = 検収の実行名::生成する(領域マスクのファイル名)?;
    let マスクの絵 = crate::sample_world_region::領域マスクを撮る(&読み戻しの書き出し先::出力ディレクトリの中に決める(
        &出力先,
        領域マスクの実行名,
    ))?;
    let 領域一覧 = 固定領域一覧を作る(&マスクの絵)?;
    let 跨ぎ一覧 = crossing::一覧を読む()?;
    let ベース名 = 出力先.join(撮影のベース名);

    let 結果 = scan::走査する(&跨ぎ一覧, &領域一覧, &ベース名)?;
    judgment::値が有限であることを確かめる(&結果.集計一覧)?;
    record::生値を書く(&計測の生値のファイル::出力ディレクトリの中の場所(&出力先), &結果.生値)?;
    record::表を表示する(&結果.集計一覧, &跨ぎ一覧);
    let 対照の行 = judgment::対照を判定する(&跨ぎ一覧, &結果.集計一覧, &ベース名)?;
    let 絵の置き場 = picture::代表点の絵を書く(&跨ぎ一覧, &結果.集計一覧, &結果.地面の段差の降順, &出力先.join(絵のベース名))?;
    Ok(format!(
        "{}。{対照の行}。代表点の絵は{}",
        record::要約を組む(&結果.集計一覧, &跨ぎ一覧, &出力先),
        絵の置き場.join("と")
    ))
}
