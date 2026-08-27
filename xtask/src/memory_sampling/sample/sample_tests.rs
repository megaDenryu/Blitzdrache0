use super::解析する;

#[test]
fn プロセスとgpuのバイト値をmibへ変換する() {
    let Some(標本) = 解析する(5.0, "1048576,2097152,3145728,1") else {
        panic!("有効なカウンター行は解析できるはず");
    };
    assert_eq!(標本.経過秒, 5.0);
    assert_eq!(標本.ワーキングセットmib, 1.0);
    assert_eq!(標本.プライベートmib, 2.0);
    assert_eq!(標本.専用ビデオメモリmib, Some(3.0));
}

#[test]
fn gpu標本なしを取得不可として保持する() {
    let Some(標本) = 解析する(5.0, "1048576,2097152,0,0") else {
        panic!("RAM標本だけでも解析できるはず");
    };
    assert_eq!(標本.専用ビデオメモリmib, None);
}
