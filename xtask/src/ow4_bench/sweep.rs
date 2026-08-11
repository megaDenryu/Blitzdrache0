//! 物量点の列を順に測って結果へ集める工程。受け取るのは物量点の列と計測条件、返すのは物量点ごとの結果である。
//! 支度(ソースアセットの生成・リリースビルド・出力先の作成・シェーダーの一時コピー)を1度だけ済ませてから、
//! 物量点ごとにアセットを焼いて反復回数だけ走らせる。何を表に出すかと折れ点の裁定は呼び出し元と人が持つ。

use std::path::{Path, PathBuf};

use super::error::物量計測エラー;
use super::{condition, point, run, validation};
use super::{シェーダーコピー先, 出力ディレクトリ, 反復回数};

pub(super) fn 測る(物量点一覧: &[usize], 条件: &condition::計測条件) -> Result<Vec<point::物量点の結果>, 物量計測エラー> {
    if !crate::gen_source_assets::生成する() {
        return Err(物量計測エラー::検証用ソースアセットを生成できなかった);
    }
    crate::release_build::計測用に構築する("ow4-bench").map_err(物量計測エラー::計測用の構築が失敗した)?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| 物量計測エラー::出力先を作れなかった {
        パス: 出力先.clone(), 誤り
    })?;
    let シェーダー入口 =
        crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先)).map_err(物量計測エラー::シェーダーの一時コピーを作れなかった)?;
    物量点一覧
        .iter()
        .map(|個体数| 一物量点を測る(&出力先, &シェーダー入口, *個体数, 条件))
        .collect()
}

fn 一物量点を測る(
    出力先: &Path,
    シェーダー入口: &Path,
    チャンクあたり個体数: usize,
    条件: &condition::計測条件,
) -> Result<point::物量点の結果, 物量計測エラー> {
    let アセットルート = 出力先.join(format!("assets_{チャンクあたり個体数}"));
    if !crate::compile_assets::地形世界を個体数指定で生成する(&アセットルート, チャンクあたり個体数) {
        return Err(物量計測エラー::物量点の実行時アセットを生成できなかった {
            チャンクあたり個体数
        });
    }
    let 検査候補数 = validation::検査する(&アセットルート, シェーダー入口, 条件)?;
    let 実行一覧 = (1..=反復回数)
        .map(|回| run::走らせる(出力先, &format!("x{チャンクあたり個体数}_{回}"), &アセットルート, シェーダー入口, 条件))
        .collect::<Result<Vec<run::一回の実行>, 物量計測エラー>>()?;
    Ok(point::物量点の結果 {
        名前: format!("チャンクあたり{チャンクあたり個体数}体"),
        チャンクあたり個体数,
        検査候補数,
        実行一覧,
    })
}
