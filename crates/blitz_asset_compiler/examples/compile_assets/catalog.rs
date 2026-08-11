//! 既知の開発用ソースアセットを安定IDと実行時形式の出力名へ対応付ける工程。
//! 世界ごとに何を焼くかは`world`が決め、ここは宣言をコンパイル時カタログとコンパイル対象一覧へ写す。
//! 対象1件の型は`compile_target`が持つ。
//!
//! 置き場と世界と2つのカタログはコンパイルのサービス型が保持しているため、この工程は何も持参されない。

use std::path::PathBuf;

use blitz_engine::{アセットID, チャンク座標};

use super::chunk_ledger::台帳での扱い;
use super::compilation::実行時アセットのコンパイル;
use super::compile_target::コンパイル対象;
use super::source_location::{self, ソースの基準};

/// ストリーミング対象でないアセットは世界の原点チャンクへ帰属させる。
/// 参照: `_doc/計画/ユビキタス言語.md`「所有チャンク」
const 原点チャンク: チャンク座標 = チャンク座標::生成する(0, 0);

/// ソースの形式と、形式ごとに要る付随の宣言は`source_kind`が持つ。宣言の書き手が同じ綴りで参照できるよう、ここから見せ直す。
pub(super) use super::source_kind::{ソース種別, 同居植生宣言};

/// 世界へ焼くアセット1件の宣言。`必須`はソースが無いときに失敗させるかどうかである。
/// `実行時へ焼く`が偽の定義は、他のアセットが素材として読むだけのソースであり、コンパイル時カタログへは載るが実行時形式は作らない。
/// `基準`は`相対パス`をどのルートから参照するかであり、宣言側は起点の実パスを持たない。
pub(super) struct アセット定義 {
    pub(super) 名前: &'static str,
    pub(super) 相対パス: &'static str,
    pub(super) 基準: ソースの基準,
    pub(super) 必須: bool,
    pub(super) 実行時へ焼く: bool,
    pub(super) 種別: ソース種別,
}

impl 実行時アセットのコンパイル {
    /// チャンク以外に焼く宣言をコンパイル時カタログへ写し、実行時形式を作る対象を並べる。
    pub(super) fn アセット宣言をカタログへ写す(&mut self) -> Result<Vec<コンパイル対象>, String> {
        let 外部ルート = source_location::外部ソースルート::解決する();
        let mut 対象一覧 = Vec::new();
        for 定義 in self.世界.アセット定義一覧() {
            let Some(ソースパス) = self.宣言のソースを開ける形で参照する(&定義, &外部ルート)? else {
                continue;
            };
            let id = アセットID::生成する(定義.名前).map_err(|誤り| 誤り.to_string())?;
            self.コンパイル時カタログ.登録する(id.clone(), ソースパス);
            if 定義.実行時へ焼く {
                対象一覧.push(コンパイル対象::生成する(
                    id,
                    原点チャンク,
                    定義.種別,
                    &self.出力ルート,
                    台帳での扱い::毎回焼く,
                ));
            }
        }
        Ok(対象一覧)
    }

    /// 置き場が無いか未取得の宣言は値なしを返して飛ばす。必須の宣言だけは失敗にする。
    fn 宣言のソースを開ける形で参照する(
        &self,
        定義: &アセット定義,
        外部ルート: &source_location::外部ソースルート,
    ) -> Result<Option<PathBuf>, String> {
        let 参照結果 = source_location::ソースパスを参照する(定義.基準, 定義.相対パス, &self.ソースルート, 外部ルート);
        let ソースパス = match 参照結果 {
            Ok(パス) => パス,
            Err(診断) => {
                println!("[compile_assets] 置き場が無いため{}をスキップ: {診断}", 定義.名前);
                return Ok(None);
            }
        };
        if ソースパス.is_file() {
            return Ok(Some(ソースパス));
        }
        if 定義.必須 {
            return Err(format!("必須ソースアセットが存在しない: {}", ソースパス.display()));
        }
        println!("[compile_assets] 未取得のためスキップ: {}", ソースパス.display());
        Ok(None)
    }
}
