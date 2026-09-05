//! 箱の塔の不変性検査(判断18・決定性)。
//! 休止の有無による1000刻み後の配置一致と、同一入力の2回実行によるビット一致を固定する。
//! 休止の有無の比較を箱1段で行うのは、判断18の静穏の4条件目(錨の置き直しなし)を足した後、2段以上の塔が1000刻みの
//! あいだ一度も休止しないためである。休止しない場面で比べても、休止が結果を変えないことを何も示さない。

use super::tower_fixture::箱の塔の場面を作る;
use crate::contact::contact_thresholds::休止と判定する接触余白;

#[test]
fn 休止の有無で千刻み後の配置が接触余白以内で一致する() {
    let (mut 工程_有効, mut 台帳_有効, 箱id一覧, _) = 箱の塔の場面を作る(1, false);
    let (mut 工程_無効, mut 台帳_無効, _, _) = 箱の塔の場面を作る(1, false);

    let mut 休止を通った = false;
    for _ in 0..1000 {
        let Ok(()) = 工程_有効.一刻み進める(&mut 台帳_有効) else {
            panic!();
        };
        let Ok(()) = 工程_無効.休止を判定せずに一刻み進める(&mut 台帳_無効) else {
            panic!();
        };
        for &id in &箱id一覧 {
            let Ok(剛体) = 台帳_有効.参照する(id) else {
                panic!();
            };
            if 剛体.実行状態().休止しているか() {
                休止を通った = true;
            }
        }
    }
    assert!(
        休止を通った,
        "休止を有効にした実行が1000刻みのあいだ一度も休止しておらず、比較が空振りしている"
    );

    let 余白 = 休止と判定する接触余白().値();
    for &id in &箱id一覧 {
        let Ok(剛体_有効) = 台帳_有効.参照する(id) else {
            panic!();
        };
        let Ok(剛体_無効) = 台帳_無効.参照する(id) else {
            panic!();
        };
        let p_有 = 剛体_有効.配置().重心の位置();
        let p_無 = 剛体_無効.配置().重心の位置();
        let dx = (p_有.x().値() - p_無.x().値()).abs();
        let dy = (p_有.y().値() - p_無.y().値()).abs();
        let dz = (p_有.z().値() - p_無.z().値()).abs();
        assert!(dx <= 余白, "xの差が余白超過: dx={dx}, 余白={余白}");
        assert!(dy <= 余白, "yの差が余白超過: dy={dy}, 余白={余白}");
        assert!(dz <= 余白, "zの差が余白超過: dz={dz}, 余白={余白}");
    }
}

#[test]
fn 同じ入力の二回の実行が全剛体の配置と速度でビット一致する() {
    let (mut 工程1, mut 台帳1, 箱id一覧, _) = 箱の塔の場面を作る(10, false);
    let (mut 工程2, mut 台帳2, _, _) = 箱の塔の場面を作る(10, false);

    for _ in 0..100 {
        let Ok(()) = 工程1.一刻み進める(&mut 台帳1) else {
            panic!();
        };
        let Ok(()) = 工程2.一刻み進める(&mut 台帳2) else {
            panic!();
        };
    }

    for &id in &箱id一覧 {
        let Ok(b1) = 台帳1.参照する(id) else {
            panic!();
        };
        let Ok(b2) = 台帳2.参照する(id) else {
            panic!();
        };
        let p1 = b1.配置().重心の位置();
        let p2 = b2.配置().重心の位置();
        assert_eq!(p1.x().値().to_bits(), p2.x().値().to_bits());
        assert_eq!(p1.y().値().to_bits(), p2.y().値().to_bits());
        assert_eq!(p1.z().値().to_bits(), p2.z().値().to_bits());
        let (v1, v2) = (b1.速度().ok(), b2.速度().ok());
        assert_eq!(v1, v2);
    }
}
