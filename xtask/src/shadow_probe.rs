//! 律速切り分けの計測バッチの入口。担当するのは、1つの軸に属する条件を子プロセスとして交互に起動し、
//! 順序と条件と生値を1つの結果へ集めることである。判定は行わず、表と生値ファイルを出して人が読む。
//!
//! 条件を交互に起動するのは、同じ条件を続けて何回も回すと機材の状態の移り変わりが条件の差に化けるためである。
//! 実測では同じ条件の実行のあいだで合計が3.13ミリ秒から4.31ミリ秒まで動いた。周回ごとに開始条件をずらすため、
//! 特定の条件だけが常に温まった状態や冷えた状態に当たることもない。
//! 解像度の軸はシャドウマップ資源を作り直すためプロセスを分ける。同一プロセス内での切り替えは要らない。
//! 生値と実行ログは軸ごとのディレクトリへ置く。軸を続けて回したときに前の軸の証拠が消えると、
//! 後から順序の影響を疑えなくなるためである。実行時アセットは世界と物量が同じなら同じものであるため軸をまたいで共有する。
//! 頂点量の軸だけは条件ごとに別の世界を読むため、その軸の条件が使う世界のぶんだけ置き場が増える。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

mod assets;
mod error;
mod parse;
mod plan;
mod record;
mod run;
mod schedule;
mod table;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use error::律速切り分けの計測エラー;

use crate::release_build::計測の生値のファイル;

const 出力ディレクトリ: &str = "target/shadow_probe";
const シェーダーコピー先: &str = "target/shadow_probe_shaders";

pub(crate) fn 影の律速切り分けを計測する(引数一覧: &[String]) -> ExitCode {
    match 走らせる(引数一覧) {
        Ok(標本一覧) => {
            table::表示する(&標本一覧);
            println!("[xtask] shadow-probe成功: 標本{}件", 標本一覧.len());
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] shadow-probe失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 走らせる(引数一覧: &[String]) -> Result<Vec<record::一標本>, 律速切り分けの計測エラー> {
    let 指定 = plan::引数を読む(引数一覧)?;
    let 条件一覧 = plan::軸の条件一覧(指定.軸);
    let 周回数 = plan::周回数を決める(指定.指定された周回数, 条件一覧.len());
    if 指定.指定された周回数.is_some_and(|指定値| 指定値 != 周回数) {
        println!("[xtask] shadow-probe: 周回数を条件数{}の倍数へ切り上げた({周回数}周回)", 条件一覧.len());
    }
    if !crate::gen_source_assets::生成する() {
        return Err(律速切り分けの計測エラー::検証用ソースアセットを生成できなかった);
    }
    crate::release_build::計測用に構築する("shadow-probe").map_err(律速切り分けの計測エラー::計測用の構築が失敗した)?;
    let ルート = PathBuf::from(出力ディレクトリ);
    let 出力先 = ルート.join(指定.軸.綴り());
    std::fs::create_dir_all(&出力先)
        .map_err(|誤り| 律速切り分けの計測エラー::出力先を作れなかった {
            パス: 出力先.clone(), 誤り
        })?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))
        .map_err(律速切り分けの計測エラー::シェーダーの一時コピーを作れなかった)?;
    let 置き場 = assets::アセットの置き場::焼く(&ルート, &条件一覧, 指定.チャンクあたり個体数)?;
    let 順序 = schedule::交互の順序(条件一覧.len(), 周回数);
    let mut 標本一覧 = Vec::with_capacity(順序.len());
    for (実行番号, 条件添字) in 順序.into_iter().enumerate() {
        let 条件 = &条件一覧[条件添字];
        let 材料 = run::実行の材料 {
            出力先: &出力先,
            アセットルート: 置き場.参照する(条件.世界)?,
            シェーダー入口: &シェーダー入口,
            指定: &指定,
            条件,
            実行番号,
        };
        標本一覧.push(run::一回走らせる(&材料)?);
    }
    record::生値を書く(&計測の生値のファイル::出力ディレクトリの中の場所(&出力先), &標本一覧)?;
    println!("[xtask] shadow-probe: 生値と実行ログは{}にある", 出力先.display());
    Ok(標本一覧)
}
