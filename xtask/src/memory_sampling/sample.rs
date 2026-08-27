//! PowerShellのプロセス情報とGPU Process Memoryカウンターを1標本へ変換する。

#[cfg(test)]
mod sample_tests;

use std::process::Command;
use std::time::Duration;

const バイト毎メビバイト: f64 = 1024.0 * 1024.0;

pub(super) struct メモリ標本 {
    経過秒: f64,
    ワーキングセットメビバイト: f64,
    プライベートメビバイト: f64,
    専用vramメビバイト: Option<f64>,
}

pub(super) fn 取得する(pid: u32, 経過: Duration) -> Option<メモリ標本> {
    let スクリプト = format!(
        "$p=Get-Process -Id {pid} -ErrorAction SilentlyContinue;if($null -eq $p){{exit 1}};$g=(Get-Counter '\\GPU Process Memory(*)\\Dedicated Usage').CounterSamples|Where-Object{{$_.InstanceName -like 'pid_{pid}_*'}}|Measure-Object CookedValue -Sum;Write-Output \"$($p.WorkingSet64),$($p.PrivateMemorySize64),$($g.Sum),$($g.Count)\""
    );
    let 出力 = Command::new("powershell").args(["-NoProfile", "-Command", &スクリプト]).output().ok()?;
    if !出力.status.success() {
        return None;
    }
    解析する(経過.as_secs_f64(), String::from_utf8(出力.stdout).ok()?.trim())
}

fn 解析する(経過秒: f64, 行: &str) -> Option<メモリ標本> {
    let mut 列 = 行.split(',');
    let ワーキングセットメビバイト = 列.next()?.parse::<f64>().ok()? / バイト毎メビバイト;
    let プライベートメビバイト = 列.next()?.parse::<f64>().ok()? / バイト毎メビバイト;
    let 専用vramバイト = 列.next()?.parse::<f64>().ok()?;
    let gpu標本数 = 列.next()?.parse::<u32>().ok()?;
    let 専用vramメビバイト = (gpu標本数 > 0).then_some(専用vramバイト / バイト毎メビバイト);
    Some(メモリ標本 {
        経過秒,
        ワーキングセットメビバイト,
        プライベートメビバイト,
        専用vramメビバイト,
    })
}

impl メモリ標本 {
    pub(super) fn 表示する(&self) {
        let vram = self.専用vramメビバイト.map_or_else(|| "取得不可".to_string(), |値| format!("{値:.2}"));
        println!(
            "{:.2},{:.2},{:.2},{vram}",
            self.経過秒, self.ワーキングセットメビバイト, self.プライベートメビバイト
        );
    }
}

/// 実行中に観測した最大値。呼出し側が表へ載せるのはこの3つだけであり、標本の列そのものは渡さない。
#[derive(Clone, Copy)]
pub(crate) struct メモリ最大 {
    pub(crate) ワーキングセットメビバイト: f64,
    pub(crate) プライベートメビバイト: f64,
    pub(crate) 専用vramメビバイト: Option<f64>, // GPUカウンターを1標本も引けなかった実行では`None`。0と区別するため真偽でなく不在で表す
}

pub(super) fn 要約を表示して最大を返す(一覧: &[メモリ標本]) -> Option<メモリ最大> {
    let (Some(先頭), Some(末尾)) = (一覧.first(), 一覧.last()) else {
        println!("メモリ推移: 標本を取得できなかった");
        return None;
    };
    println!("メモリ推移(先頭→末尾):");
    println!(
        "  ワーキングセット: {:.2} → {:.2} MiB (差 {:+.2} MiB)",
        先頭.ワーキングセットメビバイト,
        末尾.ワーキングセットメビバイト,
        末尾.ワーキングセットメビバイト - 先頭.ワーキングセットメビバイト
    );
    println!(
        "  プライベート: {:.2} → {:.2} MiB (差 {:+.2} MiB)",
        先頭.プライベートメビバイト,
        末尾.プライベートメビバイト,
        末尾.プライベートメビバイト - 先頭.プライベートメビバイト
    );
    match (先頭.専用vramメビバイト, 末尾.専用vramメビバイト) {
        (Some(開始), Some(終了)) => println!("  専用VRAM: {開始:.2} → {終了:.2} MiB (差 {:+.2} MiB)", 終了 - 開始),
        _ => println!("  専用VRAM: 取得不可"),
    }
    let 最大 = メモリ最大 {
        ワーキングセットメビバイト: 最大を取る(一覧, |標本| Some(標本.ワーキングセットメビバイト))?,
        プライベートメビバイト: 最大を取る(一覧, |標本| Some(標本.プライベートメビバイト))?,
        専用vramメビバイト: 最大を取る(一覧, |標本| 標本.専用vramメビバイト),
    };
    println!(
        "  最大: ワーキングセット {:.2} MiB / プライベート {:.2} MiB / 専用VRAM {}",
        最大.ワーキングセットメビバイト,
        最大.プライベートメビバイト,
        最大
            .専用vramメビバイト
            .map_or_else(|| "取得不可".to_string(), |値| format!("{値:.2} MiB"))
    );
    Some(最大)
}

fn 最大を取る(一覧: &[メモリ標本], 取り出す: impl Fn(&メモリ標本) -> Option<f64>) -> Option<f64> {
    一覧.iter().filter_map(取り出す).max_by(f64::total_cmp)
}
