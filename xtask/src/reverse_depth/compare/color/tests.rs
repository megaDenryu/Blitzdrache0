use super::色を比べる;
use crate::acceptance::読み戻し画像;
use crate::reverse_depth::depth_image::深度画像;

fn 色画像(中央: [u8; 3], 隣: [u8; 3]) -> Result<読み戻し画像, crate::acceptance::検収エラー> {
    let mut バイト列 = vec![0; 7 * 7 * 4];
    for 画素 in バイト列.chunks_exact_mut(4) {
        画素[3] = 255
    }
    バイト列[24 * 4..24 * 4 + 3].copy_from_slice(&中央);
    バイト列[25 * 4..25 * 4 + 3].copy_from_slice(&隣);
    読み戻し画像::テスト用に作る(7, 7, バイト列)
}

fn 一様な深度() -> Result<深度画像, crate::reverse_depth::error::逆Z検収エラー> {
    let mut バイト列 = Vec::new();
    for _ in 0..49 {
        バイト列.extend_from_slice(&0.5_f32.to_le_bytes())
    }
    深度画像::バイト列から読む(&バイト列, 49)
}

#[test]
fn 輪郭から離れた重大差を拒む() -> Result<(), Box<dyn std::error::Error>> {
    let 対照 = 色画像([2, 0, 0], [0, 0, 0])?;
    let 候補 = 色画像([0, 0, 0], [0, 0, 0])?;
    assert!(色を比べる(&対照, &候補, &一様な深度()?).is_err());
    Ok(())
}

#[test]
fn 輪郭の近くの勝者差を許す() -> Result<(), Box<dyn std::error::Error>> {
    let 対照 = 色画像([2, 0, 0], [255, 255, 255])?;
    let 候補 = 色画像([0, 0, 0], [255, 255, 255])?;
    assert!(色を比べる(&対照, &候補, &一様な深度()?).is_ok());
    Ok(())
}
