//! 間接照明の絵をオーナーが目で確かめるための入口。金属度と粗さの水準ごとに並べた球と、実アセットの小物を置いた庭を、
//! 一日の代表4時刻で本番の描画経路へ通し、目視用のPNGを4枚書き出す。
//!
//! 画素の基準値を持たないのは、この入口の目的が「間接照明の絵が現実らしいか」の判断材料を出すことであり、
//! その判断は絵を見る人が行うためである。絵は自動露出(順3-IIb)をはじめとする今後の変更で動くことが分かっており、
//! 基準値を置くと変更のたびに基準の更新が要る。機械が見るのはvalidationの指摘が0件であることと、
//! 領域マスクが地面と判定した画素が破綻防止帯に収まることだけである。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Ic-3bの実装」

mod band;
mod ground_mask;
mod run;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::day_moment::代表時刻一覧;

const 出力ディレクトリ: &str = "target/terrain_visual";

/// 地面の領域マスクを撮るときのダンプ名。パスはASCIIで保つ。
const 領域マスクのファイル名: &str = "ground_mask";

/// 目視見本の実行時形式。この世界だけの出力ルートへ焼かれる。
const 実行時形式のパス: &str = "target/terrain_visual_assets/terrain_visual.blitzasset";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] terrain-visual成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] terrain-visual失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::目視見本世界を既定で生成する() {
        return Err("目視見本のアセット生成に失敗した".to_string());
    }
    実行時形式の実在を確かめる()?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let マスクの絵 = run::描画する(&出力先.join(領域マスクのファイル名), &run::条件::領域マスク, 領域マスクのファイル名)?;
    let マスク = ground_mask::地面マスク::作る(&マスクの絵)?;
    let mut 帯の行一覧 = Vec::new();
    let mut 絵の置き場一覧 = Vec::new();
    for 時刻 in &代表時刻一覧 {
        let ダンプ先 = 出力先.join(時刻.ファイル名);
        let 条件 = run::条件::時刻の絵 {
            一日内秒: 時刻.一日内秒
        };
        let 画像 = run::描画する(&ダンプ先, &条件, 時刻.ファイル名)?;
        帯の行一覧.push(band::破綻防止帯を判定する(
            時刻.名前,
            &画像,
            &マスク,
            &地面を照らす光を選ぶ(時刻),
        )?);
        絵の置き場一覧.push(crate::raw_png::変換する(&ダンプ先)?.display().to_string());
    }
    Ok(format!(
        "5つの起動すべてでvalidationの指摘0件、地面{}画素の破綻防止帯は{}。絵は{}",
        マスク.地面画素数,
        帯の行一覧.join("、"),
        絵の置き場一覧.join("と")
    ))
}

/// 太陽が地平線より上にある時刻は方向光と間接光の両方が地面へ届き、夜は間接光だけになる。
/// 破綻防止帯が黒潰れの割合へ上限を課すかどうかがこの別で決まる。
fn 地面を照らす光を選ぶ(時刻: &crate::day_moment::代表時刻) -> band::地面を照らす光 {
    if 時刻.太陽が地平線より上か {
        band::地面を照らす光::方向光と間接光
    } else {
        band::地面を照らす光::間接光だけ
    }
}

/// 外部のアセットリポジトリが無い環境では、小物の原型が1つも解決できずコンパイルが失敗する。
/// 失敗を見落として起動へ進むと、カタログ未登録という遠い場所の失敗に化けるため、ここで名指しして落とす。
fn 実行時形式の実在を確かめる() -> Result<(), String> {
    if Path::new(実行時形式のパス).is_file() {
        return Ok(());
    }
    Err(format!(
        "{実行時形式のパス}が作られていない。外部のアセットリポジトリが見つからなかった可能性が高い(compile-assetsの出力に理由が出る)"
    ))
}
