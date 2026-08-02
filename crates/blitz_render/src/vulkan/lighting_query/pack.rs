//! 1フレームスロットぶんの照明問い合わせ資源をバイト列にする工程。受け取るのはヘッダ内容と2つのレコード列、
//! 返すのは3本のバッファへそのまま書ける固定長のバイト列か、整合しない入力の型付きの失敗である。
//!
//! ヘッダの件数が実レコード数を超える入力を通さないのは、シェーダーが件数まで読むためである。
//! 通すと、書き込まれていない領域を読む描画を作ってしまう。容量を超える入力も同じ理由でここで落とす。

use super::capacity::{局所光レコードの容量, 方向光レコードの容量};
use super::directional_content::方向光レコード内容;
use super::header_content::照明問い合わせヘッダ内容;
use super::local_content::局所光レコード内容;
use super::{directional_bytes, header_bytes, local_bytes};
use crate::error::照明問い合わせ梱包エラー;

pub(crate) const 方向光列のバイト長: usize = 方向光レコードの容量 * directional_bytes::バイト長;
pub(crate) const 局所光列のバイト長: usize = 局所光レコードの容量 * local_bytes::バイト長;

/// 3本のバッファの中身。件数を超える要素は0で埋まったまま残り、シェーダーは件数までしか読まない。
pub(crate) struct 照明問い合わせのバイト列 {
    pub(crate) ヘッダ: [u8; header_bytes::バイト長],
    pub(crate) 方向光列: [u8; 方向光列のバイト長],
    pub(crate) 局所光列: [u8; 局所光列のバイト長],
}

pub(crate) fn 組み立てる(
    ヘッダ: &照明問い合わせヘッダ内容,
    方向光一覧: &[方向光レコード内容],
    局所光一覧: &[局所光レコード内容],
) -> Result<照明問い合わせのバイト列, 照明問い合わせ梱包エラー> {
    件数を確かめる(ヘッダ.方向光件数, 方向光一覧.len(), 方向光レコードの容量, 光の種類::方向光)?;
    件数を確かめる(ヘッダ.局所光件数, 局所光一覧.len(), 局所光レコードの容量, 光の種類::局所光)?;
    let mut 方向光列 = [0u8; 方向光列のバイト長];
    for (添字, 内容) in 方向光一覧.iter().enumerate() {
        let 開始 = 添字 * directional_bytes::バイト長;
        方向光列[開始..開始 + directional_bytes::バイト長].copy_from_slice(&directional_bytes::バイト列にする(内容));
    }
    let mut 局所光列 = [0u8; 局所光列のバイト長];
    for (添字, 内容) in 局所光一覧.iter().enumerate() {
        let 開始 = 添字 * local_bytes::バイト長;
        局所光列[開始..開始 + local_bytes::バイト長].copy_from_slice(&local_bytes::バイト列にする(内容));
    }
    Ok(照明問い合わせのバイト列 {
        ヘッダ: header_bytes::バイト列にする(ヘッダ),
        方向光列,
        局所光列,
    })
}

/// エラー文の主語を分けるためだけの区別。GPUのレコードには現れない。
#[derive(Clone, Copy)]
enum 光の種類 {
    方向光,
    局所光,
}

fn 件数を確かめる(
    件数: u32, レコード数: usize, 容量: usize, 種類: 光の種類
) -> Result<(), 照明問い合わせ梱包エラー> {
    let 収まるか = usize::try_from(件数).map(|件数| 件数 <= レコード数).unwrap_or(false);
    if !収まるか || レコード数 > 容量 {
        return Err(誤りを作る(種類, 件数, レコード数, 容量));
    }
    Ok(())
}

fn 誤りを作る(種類: 光の種類, 件数: u32, レコード数: usize, 容量: usize) -> 照明問い合わせ梱包エラー {
    match 種類 {
        光の種類::方向光 => 照明問い合わせ梱包エラー::方向光の件数不整合 {
            件数, レコード数, 容量
        },
        光の種類::局所光 => 照明問い合わせ梱包エラー::局所光の件数不整合 {
            件数, レコード数, 容量
        },
    }
}
