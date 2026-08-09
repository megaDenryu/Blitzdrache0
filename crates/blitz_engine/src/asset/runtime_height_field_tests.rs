//! 高さ場の実行時形式の往復・決定性・種別の取り違え・宣言件数の過大を反証する。

#![allow(clippy::unwrap_used)]

use super::runtime_format::{
    アセット実行時形式エラー, アセット形式版, カタログを実行時形式へ格納する, 実行時アセットを格納する, 実行時アセット種別,
    実行時形式から高さ場を読む, 高さ場を実行時形式へ格納する, 高さ場実行時形式エラー,
};
use crate::asset::catalog::カタログ;
use crate::height_field::height_field_tests::検査用の高さ場を作る;

#[test]
fn 高さ場は往復して同じ値になる() {
    let 高さ場 = 検査用の高さ場を作る();
    let バイト列 = 高さ場を実行時形式へ格納する(&高さ場).unwrap();
    assert_eq!(実行時形式から高さ場を読む(&バイト列).unwrap(), 高さ場);
}

#[test]
fn 同じ高さ場は同じバイト列になる() {
    let 高さ場 = 検査用の高さ場を作る();
    let 一回目 = 高さ場を実行時形式へ格納する(&高さ場).unwrap();
    let 二回目 = 高さ場を実行時形式へ格納する(&高さ場).unwrap();
    assert_eq!(一回目, 二回目);
}

#[test]
fn 高さ場のヘッダーは種別4の版1である() {
    let バイト列 = 高さ場を実行時形式へ格納する(&検査用の高さ場を作る()).unwrap();
    let アセット = super::runtime_format::実行時アセットを開く(&バイト列).unwrap();
    assert_eq!(アセット.種別, 実行時アセット種別::高さ場);
    assert_eq!(アセット.種別.番号(), 4);
    assert_eq!(アセット.形式版, アセット形式版::V1);
}

#[test]
fn 別種別のアセットを高さ場として読むと拒む() {
    let カタログのバイト列 = カタログを実行時形式へ格納する(&カタログ::空を作る()).unwrap();
    assert!(matches!(
        実行時形式から高さ場を読む(&カタログのバイト列),
        Err(高さ場実行時形式エラー::共通形式の失敗(
            アセット実行時形式エラー::アセット種別不一致 { 期待: 4, .. }
        ))
    ));
}

#[test]
fn 標本の実体より大きい格子点数を確保前に拒む() {
    let mut 内容 = Vec::new();
    内容.extend_from_slice(&0.0_f64.to_le_bytes());
    内容.extend_from_slice(&0.0_f64.to_le_bytes());
    内容.extend_from_slice(&1.0_f32.to_le_bytes());
    内容.extend_from_slice(&60_000_u32.to_le_bytes());
    内容.extend_from_slice(&60_000_u32.to_le_bytes());
    let バイト列 = 実行時アセットを格納する(アセット形式版::V1, 実行時アセット種別::高さ場, &内容).unwrap();
    assert!(matches!(
        実行時形式から高さ場を読む(&バイト列),
        Err(高さ場実行時形式エラー::共通形式の失敗(
            アセット実行時形式エラー::件数過大 { .. }
        ))
    ));
}
