use super::{
    アセット実行時形式エラー, 実行時アセットを格納する, 実行時アセット種別, 実行時形式からカタログを読む
};

#[test]
fn カタログの重複idを型付きエラーにする() {
    let mut 内容 = 2_u32.to_le_bytes().to_vec();
    項目を書く(&mut 内容, b"fox");
    項目を書く(&mut 内容, b"fox");
    let バイト列 = 格納する(&内容);
    assert!(matches!(
        実行時形式からカタログを読む(&バイト列),
        Err(アセット実行時形式エラー::カタログID重複)
    ));
}

#[test]
fn カタログの不正utf8を型付きエラーにする() {
    let mut 内容 = 1_u32.to_le_bytes().to_vec();
    項目を書く(&mut 内容, &[0xff]);
    let バイト列 = 格納する(&内容);
    assert!(matches!(
        実行時形式からカタログを読む(&バイト列),
        Err(アセット実行時形式エラー::不正な文字列)
    ));
}

#[test]
fn カタログの過大件数を確保前に拒否する() {
    let 内容 = u32::MAX.to_le_bytes();
    let バイト列 = 格納する(&内容);
    assert!(matches!(
        実行時形式からカタログを読む(&バイト列),
        Err(アセット実行時形式エラー::件数過大 { .. })
    ));
}

fn 項目を書く(出力: &mut Vec<u8>, id: &[u8]) {
    let Ok(id長) = u32::try_from(id.len()) else {
        panic!("試験用IDが長すぎる");
    };
    出力.extend_from_slice(&id長.to_le_bytes());
    出力.extend_from_slice(id);
    出力.extend_from_slice(&1_u32.to_le_bytes());
    出力.push(b'x');
    出力.extend_from_slice(&0_u32.to_le_bytes());
    出力.extend_from_slice(&[0; 24]);
}

fn 格納する(内容: &[u8]) -> Vec<u8> {
    match 実行時アセットを格納する(実行時アセット種別::カタログ, 内容) {
        Ok(バイト列) => バイト列,
        Err(誤り) => panic!("試験用カタログを格納できなかった: {誤り}"),
    }
}
