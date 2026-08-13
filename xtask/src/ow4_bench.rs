//! OW4第5段の物量計測入口。地形と植生が同居する25チャンク世界を、密度だけを変えた3つの物量点で走らせ、
//! CPU区間・GPU時間・計数・会計・プロセス実測を採って表へ出す。折れ点の裁定は人が記録を読んで行う。
//! 原型・マテリアル・カメラ経路・LOD閾値・種・予算はすべての物量点で同じ値に固定する。変えるのは密度だけである。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「物量計測と折れ点の規則」

mod budget;
mod condition;
mod error;
mod gpu_table;
mod large_world_measure;
mod measure;
mod point;
mod run;
mod shadow_condition;
mod sweep;
mod table;
mod validation;

use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/ow4_bench";
const シェーダーコピー先: &str = "target/ow4_bench_shaders";
const 起動時シーンの綴り: &str = "terrain_origin";
const 起動時シーン: crate::acceptance::検収シーン名 = crate::acceptance::検収シーン名::生成する(起動時シーンの綴り);
const 先読み半径: &str = "2";

/// 静止先読み120、往復80、整定120の合計。先頭120フレームは集計から除かれるため、標本は往復と整定の200フレームである。
const フレーム数: &str = "320";

/// RAMとVRAMの上限。100倍の常駐25チャンクでVRAM見積が約131メガバイトになるため、予算が拘束しない値を全物量点で共有する。
/// 物量点ごとに上限を変えると、変えたのが密度だけであるという計測条件が崩れる。
const 上限バイト数: u64 = 512 * 1024 * 1024;

/// 既定の物量点。チャンクあたり400体が1倍であり、25チャンク常駐で10,000体・100,000体・1,000,000体になる。
const 既定の物量点: [usize; 3] = [400, 4_000, 40_000];

/// 同じ物量点を何回採るか。1回では実行間のノイズ帯を測れず、折れ点の判定ができない。
const 反復回数: usize = 3;

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    match condition::引数を読む(引数一覧).and_then(|読み| sweep::測る(&物量点を決める(読み.物量点一覧), &読み.条件)) {
        Ok(結果一覧) => {
            table::表示する(&結果一覧);
            budget::表示する(&結果一覧);
            let 件数 = 結果一覧.len();
            println!("[xtask] ow4-bench成功: 物量点{件数}件を各{反復回数}回、validationレイヤー有効のデバッグ実行が全物量点で0件");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] ow4-bench失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

pub(crate) fn 大規模世界を測る(指定: &crate::large_world_bench::計測指定) -> Result<(), String> {
    large_world_measure::測る(指定).map_err(|誤り| 誤り.to_string())
}

/// 物量点の指定が無ければ既定の3点を使う。折れが1倍と10倍のあいだにあったときに2倍・5倍を足して
/// 局在化する経路を、コードを変えずに開けておくためである。
fn 物量点を決める(指定: Vec<usize>) -> Vec<usize> {
    if 指定.is_empty() {
        return 既定の物量点.to_vec();
    }
    指定
}
