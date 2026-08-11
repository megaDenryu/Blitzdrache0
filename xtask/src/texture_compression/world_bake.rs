//! ブロック圧縮の検収が読む実行時形式の焼き付け。担当するのは、どの世界をどの方針でどの出力ルートへ焼くかの対応である。
//! 焼き上がったファイルの名前と読み取りは`baked_scene`が持つ。
//!
//! 方針ごとに出力ルートを分けるのは、1つの出力ルートが1つのカタログしか持てず、方針が生成物のバイト列を変えるためである。
//! 既定の出力ルート(`target/runtime_assets`等)へ焼かないのは、そちらを読む既存の入口の判定値を1つも動かさないためである。

use std::path::{Path, PathBuf};

use super::error::ブロック圧縮の検収エラー;
use crate::asset_generator::世界名;
use crate::compile_assets::texture_policy_name::{ベースカラーのブロック圧縮, 全てRGBA8};
use crate::compile_assets::方針を名指しして実行時形式を生成する;

/// 対照の板の世界を焼く先。3つ目は同じ方針で2度焼いた結果を突き合わせる決定性の検査のためだけに存在する。
const 対照の非圧縮ルート: &str = "target/texture_compression_assets_rgba8";
const 対照のブロック圧縮ルート: &str = "target/texture_compression_assets_bc1";
const 対照のブロック圧縮の再現ルート: &str = "target/texture_compression_assets_bc1_repeat";

/// DamagedHelmetを焼く先。ヘルメットは板の世界の任意アセットであるため、対照の板とは別の世界を別の出力ルートへ焼く。
const ヘルメットの非圧縮ルート: &str = "target/texture_compression_helmet_rgba8";
const ヘルメットのブロック圧縮ルート: &str = "target/texture_compression_helmet_bc1";

pub(super) struct 焼いた出力ルート {
    pub(super) 非圧縮: PathBuf,
    pub(super) ブロック圧縮: PathBuf,
}

/// 対照の板の世界を3つのルートへ焼く。返すのは絵を撮る2つのルートと、決定性の検査が読む再現ルートである。
pub(super) fn 対照の板の世界を方針違いで焼く() -> Result<(焼いた出力ルート, PathBuf), ブロック圧縮の検収エラー> {
    方針を指定して世界を焼く(世界名::ブロック圧縮の対照世界, 対照の非圧縮ルート, 全てRGBA8)?;
    方針を指定して世界を焼く(世界名::ブロック圧縮の対照世界, 対照のブロック圧縮ルート, ベースカラーのブロック圧縮)?;
    方針を指定して世界を焼く(世界名::ブロック圧縮の対照世界, 対照のブロック圧縮の再現ルート, ベースカラーのブロック圧縮)?;
    Ok((
        焼いた出力ルート {
            非圧縮: PathBuf::from(対照の非圧縮ルート),
            ブロック圧縮: PathBuf::from(対照のブロック圧縮ルート),
        },
        PathBuf::from(対照のブロック圧縮の再現ルート),
    ))
}

/// DamagedHelmetを持つ板の世界を2つのルートへ焼く。
pub(super) fn ヘルメットの世界を方針違いで焼く() -> Result<焼いた出力ルート, ブロック圧縮の検収エラー> {
    方針を指定して世界を焼く(世界名::板の世界, ヘルメットの非圧縮ルート, 全てRGBA8)?;
    方針を指定して世界を焼く(世界名::板の世界, ヘルメットのブロック圧縮ルート, ベースカラーのブロック圧縮)?;
    Ok(焼いた出力ルート {
        非圧縮: PathBuf::from(ヘルメットの非圧縮ルート),
        ブロック圧縮: PathBuf::from(ヘルメットのブロック圧縮ルート),
    })
}

fn 方針を指定して世界を焼く(
    世界: 世界名,
    出力ルート: &'static str,
    方針の名前: &'static str,
) -> Result<(), ブロック圧縮の検収エラー> {
    if 方針を名指しして実行時形式を生成する(Path::new(出力ルート), 世界, 方針の名前) {
        return Ok(());
    }
    Err(ブロック圧縮の検収エラー::世界を方針違いで焼けなかった {
        世界,
        方針の名前,
        出力ルート,
    })
}
