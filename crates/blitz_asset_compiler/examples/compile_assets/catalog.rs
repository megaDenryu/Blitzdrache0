//! 既知の開発用ソースアセットを安定IDと実行時形式の出力名へ対応付ける。
//! 世界ごとに何を焼くかは`world`が決め、ここは宣言をカタログとコンパイル対象一覧へ写す。

use std::path::{Path, PathBuf};

use blitz_engine::{アセットID, カタログ, チャンク座標};

use super::chunk_ledger::台帳での扱い;
use super::source_location::{self, ソースの基準};
use super::world::対象世界;

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

pub(super) struct コンパイル対象 {
    pub(super) id: アセットID,
    pub(super) 所有チャンク: チャンク座標,
    pub(super) 種別: ソース種別,
    pub(super) 出力パス: PathBuf,
    pub(super) 台帳での扱い: 台帳での扱い,
}

pub(super) fn 構築する(
    ソースルート: &Path, 出力ルート: &Path, 世界: 対象世界
) -> Result<(カタログ, Vec<コンパイル対象>), String> {
    let mut カタログ = カタログ::空を作る();
    let mut 対象一覧 = Vec::new();
    let 外部ルート = source_location::外部ソースルート::解決する();
    for 定義 in 世界.アセット定義一覧() {
        let ソースパス = match source_location::ソースパスを参照する(定義.基準, 定義.相対パス, ソースルート, &外部ルート)
        {
            Ok(パス) => パス,
            Err(診断) => {
                println!("[compile_assets] 置き場が無いため{}をスキップ: {診断}", 定義.名前);
                continue;
            }
        };
        if !ソースパス.is_file() {
            if 定義.必須 {
                return Err(format!("必須ソースアセットが存在しない: {}", ソースパス.display()));
            }
            println!("[compile_assets] 未取得のためスキップ: {}", ソースパス.display());
            continue;
        }
        let id = アセットID::生成する(定義.名前).map_err(|誤り| 誤り.to_string())?;
        カタログ.登録する(id.clone(), ソースパス);
        if !定義.実行時へ焼く {
            continue;
        }
        対象一覧.push(コンパイル対象 {
            id,
            所有チャンク: 原点チャンク,
            種別: 定義.種別,
            出力パス: 出力ルート.join(format!("{}.blitzasset", 定義.名前)),
            台帳での扱い: 台帳での扱い::毎回焼く,
        });
    }
    Ok((カタログ, 対象一覧))
}
