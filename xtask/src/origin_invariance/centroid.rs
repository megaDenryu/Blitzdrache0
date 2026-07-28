//! 高コントラストな無彩色標識の画素重心を測り、原点移動前後でカメラ移動への応答が一致することを判定する。

use super::compare::{不合格にする, 判定結果, 合格にする};
use super::run::読み戻し画像;

const 移動差許容画素: f64 = 0.25;

#[derive(Clone, Copy)]
struct 画素重心 {
    x: f64,
    y: f64,
}

pub(super) fn 移動を比較する(
    a: &読み戻し画像, a移動後: &読み戻し画像, b: &読み戻し画像, b移動後: &読み戻し画像
) -> 判定結果 {
    let 重心一覧 = [a, a移動後, b, b移動後].map(重心を測る);
    let [Ok(a重心), Ok(a後重心), Ok(b重心), Ok(b後重心)] = 重心一覧 else {
        let 理由 = 重心一覧
            .into_iter()
            .find_map(Result::err)
            .unwrap_or_else(|| "重心を取得できなかった".to_string());
        return 不合格にする("標識重心の移動が原点移動前後で一致する", 理由);
    };
    let a移動 = 差分(a重心, a後重心);
    let b移動 = 差分(b重心, b後重心);
    let a移動量 = 長さ(a移動);
    let b移動量 = 長さ(b移動);
    let 移動差 = 長さ((a移動.0 - b移動.0, a移動.1 - b移動.1));
    let 説明 = format!("A移動{a移動量:.4}px、B移動{b移動量:.4}px、移動ベクトル差{移動差:.4}px");
    if a移動量 > 0.0 && b移動量 > 0.0 && 移動差 <= 移動差許容画素 {
        合格にする("標識重心の移動が原点移動前後で一致する", 説明)
    } else {
        不合格にする("標識重心の移動が原点移動前後で一致する", 説明)
    }
}

fn 重心を測る(画像: &読み戻し画像) -> Result<画素重心, String> {
    let (幅, 高さ) = 寸法を読む(&画像.寸法宣言)?;
    let 期待長 = 幅
        .checked_mul(高さ)
        .and_then(|画素数| 画素数.checked_mul(4))
        .ok_or_else(|| "画像寸法が大きすぎる".to_string())?;
    if 画像.画素バイト列.len() != 期待長 {
        return Err(format!("寸法とバイト長が一致しない: {期待長} と {}", 画像.画素バイト列.len()));
    }
    let mut x合計 = 0.0;
    let mut y合計 = 0.0;
    let mut 件数 = 0.0;
    for (添字, 画素) in 画像.画素バイト列.chunks_exact(4).enumerate() {
        if 標識画素か(画素) {
            x合計 += f64::from(u32::try_from(添字 % 幅).map_err(|_| "画素Xが範囲外".to_string())?);
            y合計 += f64::from(u32::try_from(添字 / 幅).map_err(|_| "画素Yが範囲外".to_string())?);
            件数 += 1.0;
        }
    }
    if 件数 == 0.0 {
        return Err("高コントラスト標識の画素が0件だった".to_string());
    }
    Ok(画素重心 {
        x: x合計 / 件数,
        y: y合計 / 件数,
    })
}

fn 寸法を読む(宣言: &str) -> Result<(usize, usize), String> {
    let mut 要素 = 宣言.split_whitespace();
    let 幅 = 要素
        .next()
        .ok_or_else(|| "幅が無い".to_string())?
        .parse()
        .map_err(|_| "幅が不正".to_string())?;
    let 高さ = 要素
        .next()
        .ok_or_else(|| "高さが無い".to_string())?
        .parse()
        .map_err(|_| "高さが不正".to_string())?;
    Ok((幅, 高さ))
}

fn 標識画素か(画素: &[u8]) -> bool {
    let 最大 = 画素[0].max(画素[1]).max(画素[2]);
    let 最小 = 画素[0].min(画素[1]).min(画素[2]);
    最大 - 最小 <= 4 && u16::from(画素[0]) + u16::from(画素[1]) + u16::from(画素[2]) >= 384
}

fn 差分(前: 画素重心, 後: 画素重心) -> (f64, f64) {
    (後.x - 前.x, 後.y - 前.y)
}

fn 長さ(値: (f64, f64)) -> f64 {
    値.0.hypot(値.1)
}
