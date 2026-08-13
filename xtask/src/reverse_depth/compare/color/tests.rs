use super::色を比べる;
use crate::acceptance::読み戻し画像;
use crate::reverse_depth::depth_image::{深度画像, 逆向き投影の深度};

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
    深度を作る(&[100.0; 49])
}

fn 深度を作る(奥行き: &[f32]) -> Result<深度画像, crate::reverse_depth::error::逆Z検収エラー> {
    let バイト列: Vec<u8> = 奥行き.iter().flat_map(|距離| 逆向き投影の深度(*距離).to_le_bytes()).collect();
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

#[test]
fn 平面の深度勾配を輪郭として免除しない() -> Result<(), Box<dyn std::error::Error>> {
    let 対照 = 色画像([2, 0, 0], [0, 0, 0])?;
    let 候補 = 色画像([0, 0, 0], [0, 0, 0])?;
    let 奥行き: Vec<f32> = (0_u16..49).map(|番号| 100.0 + f32::from(番号 % 7)).collect();
    assert!(色を比べる(&対照, &候補, &深度を作る(&奥行き)?).is_err());
    Ok(())
}

#[test]
fn 深度の段差の近くの勝者差を許す() -> Result<(), Box<dyn std::error::Error>> {
    let 対照 = 色画像([2, 0, 0], [0, 0, 0])?;
    let 候補 = 色画像([0, 0, 0], [0, 0, 0])?;
    let 奥行き: Vec<f32> = (0_u16..49).map(|番号| if 番号 % 7 <= 3 { 100.0 } else { 200.0 }).collect();
    assert!(色を比べる(&対照, &候補, &深度を作る(&奥行き)?).is_ok());
    Ok(())
}

#[test]
fn 輪郭外の幾何画素が無い比較を拒む() -> Result<(), Box<dyn std::error::Error>> {
    let 色 = 色画像([0, 0, 0], [0, 0, 0])?;
    let 深度 = 深度を作る(&[10000.0; 49])?;
    assert!(色を比べる(&色, &色, &深度).is_err());
    Ok(())
}
