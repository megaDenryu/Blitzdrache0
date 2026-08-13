//! OW4の1実行ぶんの解析と会計を、大規模世界の固定条件3回へ適用する。

use std::path::{Path, PathBuf};

use crate::acceptance::{アプリの起動指定, 描画フレーム数, 検収の実行名};
use crate::large_world_bench::大規模世界の計測指定;

use super::error::物量計測エラー;
use super::run::起動引数で走らせる;

const 出力ディレクトリ: &str = "target/large_world_bench";
const シェーダーコピー先: &str = "target/large_world_bench_shaders";
const 反復回数: usize = 3;
const 検査名: 検収の実行名 = 検収の実行名::定数から生成する("large_world_validation");
pub(super) fn 測る(指定: &大規模世界の計測指定) -> Result<(), 物量計測エラー> {
    crate::release_build::計測用に構築する("large-world-bench").map_err(物量計測エラー::計測用の構築が失敗した)?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| 物量計測エラー::出力先を作れなかった {
        パス: 出力先.clone(), 誤り
    })?;
    let シェーダー入口 =
        crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先)).map_err(物量計測エラー::シェーダーの一時コピーを作れなかった)?;
    let 検査候補数 = デバッグ実行で検査する(指定, &シェーダー入口)?;
    let 引数一覧 = crate::large_world_bench::launch::起動引数を作る(指定, &シェーダー入口);
    let 実行一覧 = (1..=反復回数)
        .map(|回| 起動引数で走らせる(&出力先, &format!("large_world_{回}"), &引数一覧))
        .collect::<Result<Vec<_>, _>>()?;
    super::table::大規模世界を表示する(検査候補数, &実行一覧);
    Ok(())
}

fn デバッグ実行で検査する(指定: &大規模世界の計測指定, シェーダー入口: &Path) -> Result<u64, 物量計測エラー> {
    let 追加 = crate::large_world_bench::launch::ストリーミング計測の起動引数を作る(指定, シェーダー入口);
    let 参照: Vec<&str> = 追加.iter().map(String::as_str).collect();
    let フレーム数 = 描画フレーム数::生成する(指定.フレーム数);
    let 起動 = アプリの起動指定::シーンと計測の枚数を決める(指定.シーン, フレーム数).選択肢をまとめて足す(&参照);
    super::validation::起動指定で検査する(&指定.アセットルート, 検査名, フレーム数, &起動)
}
