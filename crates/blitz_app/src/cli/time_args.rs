//! 時刻に関する値引数の解析と、その設定一式。CLI層でこのファイルだけが時刻の語彙(一日内の秒・時間倍率)を扱う。
//! 触れる引数は`--time-of-day`と`--time-scale`の2つに限る。

use std::slice::Iter;

use blitz_engine::time::時間倍率;

use super::value_args::次の値を読む;
use super::起動引数エラー;

/// `--time-of-day <秒>`の値を読む。一日の始まりからの秒であり、一日の長さを超える値は翌日以降の同じ位相を指す。
pub(super) fn time_of_day引数を処理する(引数: &mut Iter<String>) -> Result<f64, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--time-of-day", 起動引数エラー::一日内時刻不正)?;
    let 秒 = 値.parse::<f64>().map_err(|_| 起動引数エラー::一日内時刻不正(値.clone()))?;
    if !秒.is_finite() || 秒 < 0.0 {
        return Err(起動引数エラー::一日内時刻不正(format!("有限かつ0以上でない: {値}")));
    }
    Ok(秒)
}

/// `--time-scale <倍率>`の値を読む。指定があると時計が進み始めるため、読み戻し画像の決定性は保てなくなる。
pub(super) fn time_scale引数を処理する(引数: &mut Iter<String>) -> Result<時間倍率, 起動引数エラー> {
    let 値 = 次の値を読む(引数, "--time-scale", 起動引数エラー::時間倍率不正)?;
    let 倍率 = 値.parse::<f64>().map_err(|_| 起動引数エラー::時間倍率不正(値.clone()))?;
    時間倍率::生成する(倍率).map_err(|誤り| 起動引数エラー::時間倍率不正(format!("{値}: {誤り}")))
}
