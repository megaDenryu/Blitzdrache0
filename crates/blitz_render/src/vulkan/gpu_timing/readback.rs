//! 前回このスロットで書いたクエリ結果を読み、パス時間の窓の表を更新する。
//! WAIT無しでAVAILABILITY付きで読み、未完(availability=0)のペアは読み飛ばす
//! (判断30。フェンス待ち後の呼び出しのため通常は全件availableのはず)。

use std::collections::HashMap;

use ash::vk;

use super::pass_time_window::パス時間の窓;

pub(super) fn 読み取る(
    device: &ash::Device,
    pool: vk::QueryPool,
    マッピング: &[(&'static str, u32)],
    タイムスタンプ周期ns: f32,
    窓表: &mut HashMap<&'static str, パス時間の窓>,
) {
    for &(名前, 開始添字) in マッピング {
        let Some(経過ミリ秒) = 一組を読み取る(device, pool, 開始添字, タイムスタンプ周期ns) else {
            continue;
        };
        窓表.entry(名前).or_insert_with(パス時間の窓::新規).追加する(経過ミリ秒);
    }
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
