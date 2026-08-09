//! ブロック圧縮の検収が読む実行時形式の焼き付け。担当するのは、どの世界をどの方針でどの出力ルートへ焼くかの対応と、
//! 焼いた生成物の格納バイト数を読み取ることである。
//!
//! 方針ごとに出力ルートを分けるのは、1つの出力ルートが1つのカタログしか持てず、方針が生成物のバイト列を変えるためである。
//! 既定の出力ルート(`target/runtime_assets`等)へ焼かないのは、そちらを読む既存の入口の判定値を1つも動かさないためである。

use std::path::{Path, PathBuf};

use crate::compile_assets::texture_policy_name::{ベースカラーのブロック圧縮, 全てRGBA8};
use crate::compile_assets::{ブロック圧縮の対照世界, 方針を名指しして実行時形式を生成する, 板の世界};

/// 対照の板の世界を焼く先。3つ目は同じ方針で2度焼いた結果を突き合わせる決定性の検査のためだけに存在する。
const 対照の非圧縮ルート: &str = "target/texture_compression_assets_rgba8";
const 対照のブロック圧縮ルート: &str = "target/texture_compression_assets_bc1";
const 対照のブロック圧縮の再現ルート: &str = "target/texture_compression_assets_bc1_repeat";

/// DamagedHelmetを焼く先。ヘルメットは板の世界の任意アセットであるため、対照の板とは別の世界を別の出力ルートへ焼く。
const ヘルメットの非圧縮ルート: &str = "target/texture_compression_helmet_rgba8";
const ヘルメットのブロック圧縮ルート: &str = "target/texture_compression_helmet_bc1";

/// 対照の板のシーン名。この安定IDが実行時形式のファイル名にもなる。
pub(super) const 対照の板のシーン名: &str = "texture_compression";
/// DamagedHelmetのシーン名。板の世界が任意アセットとして持つ既存の安定IDである。
pub(super) const ヘルメットのシーン名: &str = "helmet";

pub(super) struct 焼いた出力ルート {
    pub(super) 非圧縮: PathBuf,
    pub(super) ブロック圧縮: PathBuf,
}

/// 対照の板の世界を3つのルートへ焼く。返すのは絵を撮る2つのルートと、決定性の検査が読む再現ルートである。
pub(super) fn 対照の板の世界を方針違いで焼く() -> Result<(焼いた出力ルート, PathBuf), String> {
    方針を指定して世界を焼く(ブロック圧縮の対照世界, 対照の非圧縮ルート, 全てRGBA8)?;
    方針を指定して世界を焼く(ブロック圧縮の対照世界, 対照のブロック圧縮ルート, ベースカラーのブロック圧縮)?;
    方針を指定して世界を焼く(ブロック圧縮の対照世界, 対照のブロック圧縮の再現ルート, ベースカラーのブロック圧縮)?;
    Ok((
        焼いた出力ルート {
            非圧縮: PathBuf::from(対照の非圧縮ルート),
            ブロック圧縮: PathBuf::from(対照のブロック圧縮ルート),
        },
        PathBuf::from(対照のブロック圧縮の再現ルート),
    ))
}

/// DamagedHelmetを持つ板の世界を2つのルートへ焼く。
pub(super) fn ヘルメットの世界を方針違いで焼く() -> Result<焼いた出力ルート, String> {
    方針を指定して世界を焼く(板の世界, ヘルメットの非圧縮ルート, 全てRGBA8)?;
    方針を指定して世界を焼く(板の世界, ヘルメットのブロック圧縮ルート, ベースカラーのブロック圧縮)?;
    Ok(焼いた出力ルート {
        非圧縮: PathBuf::from(ヘルメットの非圧縮ルート),
        ブロック圧縮: PathBuf::from(ヘルメットのブロック圧縮ルート),
    })
}

fn 方針を指定して世界を焼く(世界名: &str, 出力ルート: &str, 方針の名前: &str) -> Result<(), String> {
    if 方針を名指しして実行時形式を生成する(Path::new(出力ルート), 世界名, 方針の名前) {
        return Ok(());
    }
    Err(format!("{世界名}を{方針の名前}で{出力ルート}へ焼けなかった"))
}

/// そのシーンの実行時形式1件のバイト数。テクスチャ格納バイト数の比を採るための実測値である。
pub(super) fn 実行時形式のバイト数を読む(出力ルート: &Path, シーン名: &str) -> Result<u64, String> {
    let パス = 実行時形式のパス(出力ルート, シーン名);
    let メタデータ = std::fs::metadata(&パス).map_err(|誤り| format!("{}の大きさを読めない: {誤り}", パス.display()))?;
    Ok(メタデータ.len())
}

pub(super) fn 実行時形式のパス(出力ルート: &Path, シーン名: &str) -> PathBuf {
    出力ルート.join(format!("{シーン名}.blitzasset"))
}
