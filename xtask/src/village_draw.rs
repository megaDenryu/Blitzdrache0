//! 見本の集落の絵と描画費用を見る入口。実アセットの小物を散布した世界を本番の描画経路へ通し、目視用のPNGを書き出したうえで、
//! 既存の終了時報告からフレーム時間・パス別GPU時間・描画発行数・可視個体数を出す。
//!
//! 画素の合否判定を置かないのは、この入口の目的が「絵として成立しているか」の判断材料を出すことであり、
//! その判断は親エージェントの目視が行うためである。機械が見るのはvalidationが0件であることだけである。
//! 参照: `_doc/設計/Blenderアセット運用.md`「段3: 小物の量産と見本の集落」

mod error;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use error::見本の集落の撮影エラー;

use crate::world_setup::検収世界の用意;

const 出力ディレクトリ: &str = "target/village_draw";

/// 集落の実行時形式。この世界だけの出力ルートへ焼かれる。
///
/// 外部のアセットリポジトリが無い環境では、小物の原型が1つも解決できずコンパイルが失敗する。
/// 失敗を見落として起動へ進むと、カタログ未登録という遠い場所の失敗に化けるため、焼いた直後に実在を確かめる。
const 検収世界: 検収世界の用意 = 検収世界の用意::生成する("見本の集落", "target/village_assets/prop_village.blitzasset");

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] village-draw成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] village-draw失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, 見本の集落の撮影エラー> {
    検収世界
        .焼き上がりを確かめる(crate::gen_source_assets::生成する() && crate::compile_assets::見本の集落世界を既定で生成する())
        .map_err(見本の集落の撮影エラー::検収世界を用意できなかった)?;
    let 実行環境 = run::実行環境を作る(PathBuf::from(出力ディレクトリ))?;
    let 実行 = 実行環境.描いて読み戻す(run::集落の実行名, &run::起動指定を組み立てる())?;
    let png = 実行.書き出し先().目視用の絵へ変換する()?;
    run::報告を書き出す(実行.報告());
    Ok(format!("本番経路の絵は{}", png.display()))
}
