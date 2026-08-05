//! 遠方環境の更新を間引く区間の境界で絵に出る段差を、一日ぶん全部実測する入口。
//! 目視見本の庭を、境界を最初に跨いだ時刻へ固定したまま、遠方環境を焼く太陽天頂区間だけ下側と上側へ
//! 差し替えた対の圧縮前HDRで撮り、固定領域(地面の拡散と粗さ水準別の球)ごとに差を集計する。
//!
//! 判定値を1つも持たないのは、上限の制定がこの実測を読んでから行われるためである(ヒストグラム範囲の制定と同じ様式)。
//! 機械が見るのはvalidationの指摘が0件であることと、報告する値がすべて有限であることだけである。
//!
//! 実時刻を境界の前後へ動かさないのは、直接光と空背景の連続変化が差へ混ざると間接照明の段差だけを測れなくなるためである。
//! 境界の一覧は`--report-sun-zenith-boundaries`が出す本番の導出から読む。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「間引きの実装設計」

mod boundary;
mod difference;
mod hdr_image;
mod measure;
mod record;
mod run;
mod summary;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::sample_world_region::{固定領域, 固定領域一覧を作る};
use summary::領域の集計;

const 出力ディレクトリ: &str = "target/ibl_step";
const 生値ファイル名: &str = "raw.tsv";
/// 領域マスクを撮るときのダンプ名。パスはASCIIで保つ。
const 領域マスクのファイル名: &str = "region_mask";
/// 撮影のダンプのベース名。境界の識別と側が後ろへ付く。
const 撮影のベース名: &str = "shot";
/// 1回の起動で撮る境界の本数。1本が対の2枚で約30メガバイトを占めるため、1束の生の画像は約0.6ギガバイトに収まる。
const 一束の境界数: usize = 20;

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
    let マスクの絵 = crate::sample_world_region::領域マスクを撮る(&出力先.join(領域マスクのファイル名), 領域マスクのファイル名)?;
    let 領域一覧 = 固定領域一覧を作る(&マスクの絵)?;
    let 境界一覧 = boundary::一覧を読む()?;
    let ベース名 = 出力先.join(撮影のベース名);

    let mut 集計一覧: Vec<領域の集計> = 領域一覧.iter().map(|領域| 領域の集計::新規(領域.名前.clone(), 画素数(領域))).collect();
    let mut 生値 = record::見出し();
    for 束の先頭 in (0..境界一覧.len()).step_by(一束の境界数) {
        let 本数 = 一束の境界数.min(境界一覧.len() - 束の先頭);
        run::一束を撮る(束の先頭, 本数, &ベース名)?;
        for 境界 in &境界一覧[束の先頭..束の先頭 + 本数] {
            measure::境界を測る(境界, &領域一覧, &mut 集計一覧, &mut 生値, &ベース名)?;
            run::撮影を捨てる(&ベース名, 境界.上側の区間識別);
        }
    }
    値が有限であることを確かめる(&集計一覧)?;
    record::生値を書く(&出力先.join(生値ファイル名), &生値)?;
    record::表を表示する(&集計一覧);
    Ok(record::要約を組む(&集計一覧, 境界一覧.len(), &出力先))
}

/// 機械判定の1つ。報告する値がすべて有限であることを見る。上限は持たない。
fn 値が有限であることを確かめる(集計一覧: &[領域の集計]) -> Result<(), String> {
    for 集計 in 集計一覧 {
        for (呼び名, 値) in [
            ("明るさの最大", 集計.明るさの最大.値),
            ("明るさのp95の最大", 集計.明るさのp95の最大.値),
            ("色の最大", 集計.色の最大.値),
            ("色のp95の最大", 集計.色のp95の最大.値),
        ] {
            if !値.is_finite() {
                return Err(format!("{}の{呼び名}が有限でない: {値}", 集計.名前));
            }
        }
    }
    Ok(())
}

fn 画素数(領域: &固定領域) -> u64 {
    u64::try_from(領域.画素添字一覧.len()).unwrap_or(u64::MAX)
}
