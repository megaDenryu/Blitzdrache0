//! OW4の1実行ぶんの解析と会計を、大規模世界の固定条件3回へ適用する。

use std::path::{Path, PathBuf};

use crate::acceptance::{
    アプリの起こし方, アプリの起動指定, 世界を読ませて報告を採る実行環境, 判定の名前, 実行時アセットルート, 描画フレーム数, 検収の実行名,
    検収シーン名,
};
use crate::large_world_bench::計測指定;

use super::error::物量計測エラー;
use super::run::{一回の実行, 起動引数で走らせる};

const 出力ディレクトリ: &str = "target/large_world_bench";
const シェーダーコピー先: &str = "target/large_world_bench_shaders";
const 反復回数: usize = 3;
const 検査名: 検収の実行名 = 検収の実行名::定数から生成する("large_world_validation");
const 検査シーン: 検収シーン名 = 検収シーン名::生成する("terrain_fox_tour");
const 検証層の指摘: 判定の名前 = 判定の名前::定数から生成する("大規模世界計測のvalidation指摘");

pub(super) fn 測る(指定: &計測指定) -> Result<(), 物量計測エラー> {
    crate::release_build::計測用に構築する("large-world-bench").map_err(物量計測エラー::計測用の構築が失敗した)?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| 物量計測エラー::出力先を作れなかった {
        パス: 出力先.clone(), 誤り
    })?;
    let シェーダー入口 =
        crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先)).map_err(物量計測エラー::シェーダーの一時コピーを作れなかった)?;
    let 検査候補数 = validation検査(指定, &シェーダー入口)?;
    let 引数一覧 = crate::large_world_bench::launch::起動引数を作る(指定, &シェーダー入口);
    let 実行一覧 = (1..=反復回数)
        .map(|回| 起動引数で走らせる(&出力先, &format!("large_world_{回}"), &引数一覧))
        .collect::<Result<Vec<_>, _>>()?;
    表示する(検査候補数, &実行一覧);
    Ok(())
}

fn validation検査(指定: &計測指定, シェーダー入口: &Path) -> Result<u64, 物量計測エラー> {
    let 環境 = 世界を読ませて報告を採る実行環境::作る(
        アプリの起こし方::毎回cargoに構築させて起動する,
        実行時アセットルート::パスから生成する(指定.アセットルート.clone()),
    );
    let 追加 = crate::large_world_bench::launch::追加起動引数を作る(指定, シェーダー入口);
    let 参照: Vec<&str> = 追加.iter().map(String::as_str).collect();
    let 起動 =
        アプリの起動指定::シーンと計測の枚数を決める(検査シーン, 描画フレーム数::生成する(指定.フレーム数)).選択肢をまとめて足す(&参照);
    let 報告 = 環境.報告を採る(検査名, &起動)?;
    let 計数 = crate::report_parse::取り出す(&報告)?;
    検証層の指摘.零件であることを課す(計数.validation件数)?;
    Ok(計数.シーン.候補数)
}

fn 表示する(検査候補数: u64, 実行一覧: &[一回の実行]) {
    println!("\n大規模世界: validation検査候補数{検査候補数}、反復{}回", 実行一覧.len());
    for (番号, 実行) in 実行一覧.iter().enumerate() {
        println!("  実行{}:", 番号 + 1);
        区間を表示する("可視個体の選別", 実行.区間.可視個体の選別);
        区間を表示する("作業領域更新", 実行.区間.作業領域更新);
        区間を表示する("描画記録・送信・提示", 実行.区間.描画記録送信提示);
        区間を表示する("フレーム間隔", 実行.区間.フレーム間隔);
        println!(
            "    GPU シーン {:.4} / シャドウ {:.4} / 定常合計 {:.4} ms",
            実行.gpu.シーン描画ms, 実行.gpu.シャドウ合計ms, 実行.gpu.定常合計ms
        );
        println!(
            "    計数 候補{} 可視{} 発行{}",
            実行.計数.シーン.候補数, 実行.計数.シーン.可視数, 実行.計数.シーン.発行数
        );
        println!(
            "    会計 RAM{} VRAM{} 読込{}件/{}bytes 解除{}件",
            実行.要約.最大ramバイト数, 実行.要約.最大vramバイト数, 実行.要約.ディスク読込件数, 実行.要約.ディスク読込バイト数, 実行.要約.gpu解除件数
        );
    }
}

fn 区間を表示する(名前: &str, 分布: super::measure::区間分布) {
    println!("    CPU {名前} p50 {:.4} / p95 {:.4} / p99 {:.4} ms", 分布.p50ms, 分布.p95ms, 分布.p99ms);
}
