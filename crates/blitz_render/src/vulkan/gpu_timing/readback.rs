//! 前回このスロットで書いたクエリ結果を読み、パス時間の窓の表を更新する。
//! WAIT無しでAVAILABILITY付きで読み、未完(availability=0)のペアは読み飛ばす
//! (判断30。フェンス待ち後の呼び出しのため通常は全件availableのはず)。
//!
//! 同じパス名がフレーム内に何度も現れたら、その回のぶんを足して1つの標本にする。
//! 鏡面畳込み生成は粗さの段ごとにパスを積むため、回ごとに1標本を入れると窓の中身が段の混じった列になり、中央値も95パーセンタイルも「そのフレームでその区間に費やした時間」を答えなくなる(段ごとに大きさが違うため混じった列の中央値はどの段の値でもない)。
//! 窓の1標本を1フレームぶんに保つことで、区間の分位がそのまま更新1回の費用になる。

use std::collections::HashMap;

use ash::vk;

use super::composite_interval::{self, 合成区間の宣言};
use super::pass_time_window::パス時間の窓;

pub(super) fn 読み取る(
    device: &ash::Device,
    pool: vk::QueryPool,
    マッピング: &[(&'static str, u32)],
    タイムスタンプ周期ns: f32,
    合成区間一覧: &[合成区間の宣言],
    窓表: &mut HashMap<&'static str, パス時間の窓>,
) {
    let 読み値一覧: Vec<(&'static str, Option<f64>)> = マッピング
        .iter()
        .map(|&(名前, 開始添字)| (名前, 一組を読み取る(device, pool, 開始添字, タイムスタンプ周期ns)))
        .collect();
    let フレーム内の合計 = フレーム内で合算する(&読み値一覧);
    let 合成 = composite_interval::適用する(合成区間一覧, &フレーム内の合計);
    for (名前, 合計ミリ秒) in フレーム内の合計.into_iter().chain(合成) {
        窓表.entry(名前).or_insert_with(パス時間の窓::新規).追加する(合計ミリ秒);
    }
}

/// 同じ名前の回を足して、名前ごとに1フレームぶんの合計を返す。
/// 回が1つでも読めなかった名前は、そのフレームのぶんを丸ごと捨てる。
/// 読めた回だけを足した合計はそのフレームの費用を過小に見せ、読み飛ばしたことが値から分からなくなるためである。
/// 並びは最初に現れた順を保つ(グラフの宣言順がそのまま報告の並びになる)。
fn フレーム内で合算する(読み値一覧: &[(&'static str, Option<f64>)]) -> Vec<(&'static str, f64)> {
    let mut 合計一覧: Vec<(&'static str, Option<f64>)> = Vec::new();
    for &(名前, 経過ミリ秒) in 読み値一覧 {
        match 合計一覧.iter_mut().find(|(既存の名前, _)| *既存の名前 == 名前) {
            Some((_, 合計)) => *合計 = 合計.zip(経過ミリ秒).map(|(積み上げ, 値)| 積み上げ + 値),
            None => 合計一覧.push((名前, 経過ミリ秒)),
        }
    }
    合計一覧.into_iter().filter_map(|(名前, 合計)| 合計.map(|値| (名前, 値))).collect()
}

/// `開始添字`・`開始添字+1`の2クエリを読み、経過ミリ秒を求める。availability無し、
/// 逆転(終了<開始)、u32範囲を超えるtick差分のいずれかは「計測できなかった」として
/// `None`を返す(無言のゼロ値・無言の巨大値を返さない)。
fn 一組を読み取る(device: &ash::Device, pool: vk::QueryPool, 開始添字: u32, タイムスタンプ周期ns: f32) -> Option<f64> {
    let mut 結果: [[u64; 2]; 2] = [[0; 2]; 2];
    // 安全性: poolは生成済みで、開始添字・開始添字+1はプール容量内(呼び出し元が保証)。
    let 読み取り結果 = unsafe {
        device.get_query_pool_results(
            pool,
            開始添字,
            &mut 結果,
            vk::QueryResultFlags::TYPE_64 | vk::QueryResultFlags::WITH_AVAILABILITY,
        )
    };
    if 読み取り結果.is_err() {
        return None;
    }

    let [開始値, 開始可用] = 結果[0];
    let [終了値, 終了可用] = 結果[1];
    if 開始可用 == 0 || 終了可用 == 0 || 終了値 < 開始値 {
        return None;
    }

    let 差分tick = 終了値 - 開始値;
    let 差分tick_u32 = u32::try_from(差分tick).ok()?;
    let 差分ns = f64::from(差分tick_u32) * f64::from(タイムスタンプ周期ns);
    Some(差分ns / 1_000_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 同じ名前の回を足して一標本にする() {
        let 読み値 = [("鏡面畳込み生成", Some(0.10)), ("鏡面畳込み生成", Some(0.20)), ("シーン描画", Some(1.0))];
        let 合計 = フレーム内で合算する(&読み値);
        assert_eq!(合計.len(), 2);
        assert_eq!(合計[0].0, "鏡面畳込み生成");
        assert!((合計[0].1 - 0.30).abs() < 1.0e-9, "{}", 合計[0].1);
        assert_eq!(合計[1], ("シーン描画", 1.0));
    }

    #[test]
    fn 読めない回があった名前はそのフレームを捨てる() {
        let 読み値 = [("鏡面畳込み生成", Some(0.10)), ("鏡面畳込み生成", None), ("シーン描画", Some(1.0))];
        let 合計 = フレーム内で合算する(&読み値);
        assert_eq!(合計, vec![("シーン描画", 1.0)]);
    }
}
