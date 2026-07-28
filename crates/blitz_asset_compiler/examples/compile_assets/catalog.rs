//! 既知の開発用ソースアセットを安定IDと実行時形式の出力名へ対応付ける。
//! 世界ごとに何を焼くかは`world`が決め、ここは宣言をカタログとコンパイル対象一覧へ写す。

use std::path::{Path, PathBuf};

use blitz_engine::{アセットID, カタログ, チャンク座標};

use super::world::対象世界;

/// ストリーミング対象でないアセットは世界の原点チャンクへ帰属させる。
/// 参照: `_doc/計画/ユビキタス言語.md`「所有チャンク」
const 原点チャンク: チャンク座標 = チャンク座標::生成する(0, 0);

/// コンパイル対象のソースが何の形式で書かれているか。読み方が形式ごとに違うため、判別共用体で持つ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ソース種別 {
    /// glTF(.gltf/.glb)で書かれたシーン。
    Gltfシーン,
    高さ格子,
    /// glTFを原型として読み、決定的に生成した配置でインスタンス群を焼く。
    植生 {
        個体数: usize,
    },
    /// 同じ原型から、可視判定の検収用の固定配置と影を受ける床を焼く。
    植生可視判定,
    /// 同じ原型から、恒等でないTRSを与えた個体1体と影を受ける床を焼く。
    植生単一個体,
}

/// 世界へ焼くアセット1件の宣言。`必須`はソースが無いときに失敗させるかどうかである。
pub(super) struct アセット定義 {
    pub(super) 名前: &'static str,
    pub(super) 相対パス: &'static str,
    pub(super) 必須: bool,
    pub(super) 種別: ソース種別,
}

pub(super) struct コンパイル対象 {
    pub(super) id: アセットID,
    pub(super) 所有チャンク: チャンク座標,
    pub(super) 種別: ソース種別,
    pub(super) 出力パス: PathBuf,
}

pub(super) fn 構築する(
    ソースルート: &Path, 出力ルート: &Path, 世界: 対象世界
) -> Result<(カタログ, Vec<コンパイル対象>), String> {
    let mut カタログ = カタログ::空を作る();
    let mut 対象一覧 = Vec::new();
    for 定義 in 世界.アセット定義一覧() {
        let ソースパス = ソースルート.join(定義.相対パス);
        if !ソースパス.is_file() {
            if 定義.必須 {
                return Err(format!("必須ソースアセットが存在しない: {}", ソースパス.display()));
            }
            println!("[compile_assets] 未取得のためスキップ: {}", ソースパス.display());
            continue;
        }
        let id = アセットID::生成する(定義.名前).map_err(|誤り| 誤り.to_string())?;
        カタログ.登録する(id.clone(), ソースパス);
        対象一覧.push(コンパイル対象 {
            id,
            所有チャンク: 原点チャンク,
            種別: 定義.種別,
            出力パス: 出力ルート.join(format!("{}.blitzasset", 定義.名前)),
        });
    }
    Ok((カタログ, 対象一覧))
}
