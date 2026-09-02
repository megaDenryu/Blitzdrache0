//! 刻みの所要時間の分布の検査。分位点が度数を下から積んだ区分の上端(最大値を越えない)であることと、1度も刻んでいない分布が要約を持たないことを固定する。

use std::time::Duration;

use super::step_time_distribution::{刻みの所要時間の分布, 刻みの所要時間の要約};

#[test]
fn 分位点は度数を下から積んだ区分の上端である() {
    let mut 分布 = 刻みの所要時間の分布::まだ刻んでいない分布を作る();
    for マイクロ秒 in [10_u64, 20, 30, 40, 50, 60, 70, 80, 90, 1000] {
        分布.記録する(Duration::from_micros(マイクロ秒));
    }
    let 刻みの所要時間の要約::刻んだ {
        回数,
        最小,
        中央値,
        九十九パーセント点,
        最大,
        ..
    } = 分布.要約する()
    else {
        panic!("10刻みを記録した分布が刻んでいない");
    };
    assert_eq!(回数, 10);
    assert_eq!(最小, Duration::from_micros(10));
    assert_eq!(中央値, Duration::from_micros(51));
    assert_eq!(九十九パーセント点, Duration::from_micros(1000));
    assert_eq!(最大, Duration::from_micros(1000));
}

#[test]
fn 一度も刻んでいない分布は要約を持たない() {
    assert_eq!(
        刻みの所要時間の分布::まだ刻んでいない分布を作る().要約する(),
        刻みの所要時間の要約::まだ刻んでいない
    );
}
