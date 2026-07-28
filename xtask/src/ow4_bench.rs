//! OW4第5段の物量計測入口。地形と植生が同居する25チャンク世界を、密度だけを変えた3つの物量点で走らせ、
//! CPU区間・GPU時間・計数・会計・プロセス実測を採って表へ出す。折れ点の裁定は人が記録を読んで行う。
//! 原型・マテリアル・カメラ経路・LOD閾値・種・予算はすべての物量点で同じ値に固定する。変えるのは密度だけである。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「物量計測と折れ点の規則」

mod measure;
mod point;
mod run;
mod section_parse;
mod table;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/ow4_bench";
const シェーダーコピー先: &str = "target/ow4_bench_shaders";
const 実行ファイル: &str = "target/release/blitz_app.exe";
const 起動時シーン: &str = "terrain_origin";
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
    match 物量点を決める(引数一覧).and_then(|物量点一覧| 計測する(&物量点一覧)) {
        Ok(結果一覧) => {
            table::表示する(&結果一覧);
            println!(
                "[xtask] ow4-bench成功: 物量点{}件を各{反復回数}回、validation報告は全実行0件",
                結果一覧.len()
            );
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] ow4-bench失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

/// 引数なしなら既定の3点、引数があればチャンクあたり個体数の列として読む。折れが1倍と10倍のあいだにあったときに
/// 2倍・5倍を足して局在化する経路を、コードを変えずに開けておくためである。
fn 物量点を決める(引数一覧: &[String]) -> Result<Vec<usize>, String> {
    if 引数一覧.is_empty() {
        return Ok(既定の物量点.to_vec());
    }
    引数一覧
        .iter()
        .map(|語| match 語.parse::<usize>() {
            Ok(個体数) if 個体数 > 0 => Ok(個体数),
            Ok(_) => Err("チャンクあたり個体数は1以上である必要がある".to_string()),
            Err(誤り) => Err(format!("チャンクあたり個体数を数として読めない: {誤り}")),
        })
        .collect()
}

fn 計測する(物量点一覧: &[usize]) -> Result<Vec<point::物量点の結果>, String> {
    if !crate::gen_source_assets::生成する() {
        return Err("検証用ソースアセットの生成に失敗した".to_string());
    }
    if !crate::release_build::実行する("ow4-bench") {
        return Err("リリースビルドに失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;
    物量点一覧
        .iter()
        .map(|個体数| 一物量点を測る(&出力先, &シェーダー入口, *個体数))
        .collect()
}

fn 一物量点を測る(
    出力先: &Path, シェーダー入口: &Path, チャンクあたり個体数: usize
) -> Result<point::物量点の結果, String> {
    let アセットルート = 出力先.join(format!("assets_{チャンクあたり個体数}"));
    if !crate::compile_assets::地形世界を個体数指定で生成する(&アセットルート, チャンクあたり個体数) {
        return Err(format!("チャンクあたり{チャンクあたり個体数}体の実行時アセット生成に失敗した"));
    }
    let 実行一覧 = (1..=反復回数)
        .map(|回| run::走らせる(出力先, &format!("x{チャンクあたり個体数}_{回}"), &アセットルート, シェーダー入口))
        .collect::<Result<Vec<run::一回の実行>, String>>()?;
    Ok(point::物量点の結果 {
        名前: format!("チャンクあたり{チャンクあたり個体数}体"),
        チャンクあたり個体数,
        実行一覧,
    })
}
