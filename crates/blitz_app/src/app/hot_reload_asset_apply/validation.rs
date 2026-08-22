//! 公開済み一式のうち、状態を変えずに確定できるチャンク一辺・高さ場・地表の層のタイルを先に作る工程。

use super::{アプリ, 公開済みの実行時アセット一式};
use crate::app::scene_load::地表の層のタイル一式;

pub(super) struct 反映準備 {
    pub(super) 高さ場: crate::game::ゲーム用高さ場,
    pub(super) 一辺: Option<blitz_engine::チャンク一辺>,
    pub(super) 地表の層のタイル: 地表の層のタイル一式,
}

pub(super) fn 反映前の検査を通す(アプリ: &アプリ, 一式: &公開済みの実行時アセット一式) -> Option<反映準備> {
    if let Some(配線) = &アプリ.ストリーミング
        && let Err(誤り) = 配線.チャンク目録の一辺を検査する(&一式.チャンク目録)
    {
        eprintln!("[hot-reload] 実行時アセット一式を採用しなかった: {誤り}");
        return None;
    }
    let 高さ場 = match アプリ.ゲーム配線.カタログから高さ場を読み込む(&一式.カタログ) {
        Ok(高さ場) => 高さ場,
        Err(誤り) => {
            eprintln!("[hot-reload] 新しい高さ場を読み込めなかった: {誤り}");
            return None;
        }
    };
    let 地表の層のタイル = match 地表の層のタイル一式::読み口から作る(&一式.地表層テクスチャ集) {
        Ok(タイル) => タイル,
        Err(誤り) => {
            eprintln!("[hot-reload] 新しい地表層テクスチャ集を層のタイルへ写せなかった: {誤り}");
            return None;
        }
    };
    Some(反映準備 {
        高さ場,
        一辺: アプリ.ストリーミング.as_ref().map(super::super::streaming::ストリーミング配線::一辺),
        地表の層のタイル,
    })
}
