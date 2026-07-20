//! CLI引数の解析。CLIはプロセス境界のため、引数名はASCIIのまま扱う。

use std::path::PathBuf;

use crate::error::起動エラー;

const 既定シェーダー監視パス: &str = "shaders/scene.slang";
const 既定シーン名: &str = "quad";
const 既定アセットルート: &str = "assets";

/// 起動時に指定できる実行モード。
#[derive(Debug, Clone, Copy)]
pub(crate) enum 起動モード {
    /// ユーザーが閉じるまで無期限に実行する。
    無期限実行,
    /// 指定フレーム数を描画したら自動終了する(DoDのスモーク検証用)。
    スモーク実行 { フレーム数: u32 },
}

/// CLI引数から得た起動設定一式。
pub(crate) struct 起動設定 {
    pub(crate) モード: 起動モード,
    /// ホットリロードの監視対象となるエントリファイル。既定は`shaders/scene.slang`
    /// (存在しなければ監視無効)。`import`で参照される他の.slangファイル
    /// (`pbr.slang`等)はこのエントリファイルと同じディレクトリから解決される
    /// ため、個別指定はしない。ディレクトリ全体をmtime走査で監視する。
    pub(crate) シェーダー監視パス: PathBuf,
    /// 表示するシーンのアセットID。既定は`quad`(常に存在し決定的)。
    pub(crate) シーン名: String,
    /// カタログの各アセットパスの基準ディレクトリ。既定は`assets`。
    pub(crate) アセットルート: PathBuf,
    /// `--unlit`指定でfalse。既定はtrue(PBRライティング有効、判断26)。
    pub(crate) ライティング有効: bool,
    /// `--particles`指定でtrue。既定はfalse(GPU粒子トイ、判断29)。
    pub(crate) 粒子有効: bool,
    /// `--report-gpu-times`指定でtrue。既定はfalse(パス別GPU時間の終了時コンソール出力、判断30)。
    pub(crate) gpu時間報告: bool,
}

/// `--frames N` `--shader-source <path>` `--scene <id>` `--asset-root <dir>`
/// `--unlit` `--particles` `--report-gpu-times` を解析する。`--shader-source`は
/// 監視・再コンパイル対象のエントリファイルを指す。`import`先の他ファイルは常に
/// エントリと同じディレクトリから解決するため、このオプションでの指定は不要。
pub(crate) fn 引数を解析する(引数一覧: &[String]) -> Result<起動設定, 起動エラー> {
    let mut モード = 起動モード::無期限実行;
    let mut シェーダー監視パス = PathBuf::from(既定シェーダー監視パス);
    let mut シーン名 = 既定シーン名.to_string();
    let mut アセットルート = PathBuf::from(既定アセットルート);
    let mut ライティング有効 = true;
    let mut 粒子有効 = false;
    let mut gpu時間報告 = false;

    let mut 引数 = 引数一覧.iter();
    while let Some(引数値) = 引数.next() {
        match 引数値.as_str() {
            "--frames" => {
                let 値 = 引数.next().ok_or_else(|| {
                    起動エラー::フレーム数引数不正("--framesに値が指定されていない".to_string())
                })?;
                let フレーム数 = 値
                    .parse::<u32>()
                    .map_err(|_| 起動エラー::フレーム数引数不正(値.clone()))?;
                モード = 起動モード::スモーク実行 { フレーム数 };
            }
            "--shader-source" => {
                let 値 = 引数.next().ok_or_else(|| {
                    起動エラー::シェーダーソース引数不正(
                        "--shader-sourceに値が指定されていない".to_string(),
                    )
                })?;
                シェーダー監視パス = PathBuf::from(値);
            }
            "--scene" => {
                let 値 = 引数.next().ok_or_else(|| {
                    起動エラー::シーン名引数不正("--sceneに値が指定されていない".to_string())
                })?;
                シーン名 = 値.clone();
            }
            "--asset-root" => {
                let 値 = 引数.next().ok_or_else(|| {
                    起動エラー::アセットルート引数不正("--asset-rootに値が指定されていない".to_string())
                })?;
                アセットルート = PathBuf::from(値);
            }
            "--unlit" => {
                ライティング有効 = false;
            }
            "--particles" => {
                粒子有効 = true;
            }
            "--report-gpu-times" => {
                gpu時間報告 = true;
            }
            _ => {}
        }
    }

    Ok(起動設定 { モード, シェーダー監視パス, シーン名, アセットルート, ライティング有効, 粒子有効, gpu時間報告 })
}
