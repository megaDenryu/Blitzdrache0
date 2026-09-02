//! 刻みの所要時間の分布: 刻みを回す工程が測った1刻みのCPU時間を、1マイクロ秒幅の度数として貯め、回数・合計・最小・最大・
//! 中央値(p50)・99パーセント点(p99)を要約する器。性能予算の枠(世界の形への問い合わせ)の正式化が読む。
//!
//! 標本を全部貯めずに度数で持つのは、無期限の実行でも容量が育たないためである(度数の器は4097個の固定長)。
//! 分位点は度数を下から積み、標本数×割合に達した区分の上端を答える。1マイクロ秒の刻みの粗さは、枠が0.1ミリ秒の桁である
//! ことに対して十分に細かい。上端の区分を越えた所要時間は最後の区分へ数え、その分位点は最大値で答える。

use std::time::Duration;

const 区分の幅マイクロ秒: u64 = 1;
const 区分の数: usize = 4097; // 4096マイクロ秒までを1マイクロ秒ごとに、それより長い所要時間を最後の1つに数える

pub(super) struct 刻みの所要時間の分布 {
    度数: Vec<u32>,
    回数: u32,
    合計: Duration,
    最小: Duration,
    最大: Duration,
}

// 分布の要約。1度も刻んでいない状態をそのまま枝で持つ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 刻みの所要時間の要約 {
    まだ刻んでいない,
    刻んだ {
        回数: u32,
        合計: Duration,
        最小: Duration,
        中央値: Duration,
        九十九パーセント点: Duration,
        最大: Duration,
    },
}

impl 刻みの所要時間の分布 {
    pub(super) fn まだ刻んでいない分布を作る() -> Self {
        Self {
            度数: vec![0; 区分の数],
            回数: 0,
            合計: Duration::ZERO,
            最小: Duration::MAX,
            最大: Duration::ZERO,
        }
    }

    pub(super) fn 記録する(&mut self, 所要時間: Duration) {
        self.回数 = self.回数.saturating_add(1);
        self.合計 = self.合計.saturating_add(所要時間);
        self.最小 = self.最小.min(所要時間);
        self.最大 = self.最大.max(所要時間);
        let 区分 = usize::try_from(所要時間.as_micros() / u128::from(区分の幅マイクロ秒)).unwrap_or(区分の数 - 1);
        let 添字 = 区分.min(区分の数 - 1);
        self.度数[添字] = self.度数[添字].saturating_add(1);
    }

    pub(super) fn 要約する(&self) -> 刻みの所要時間の要約 {
        if self.回数 == 0 {
            return 刻みの所要時間の要約::まだ刻んでいない;
        }
        刻みの所要時間の要約::刻んだ {
            回数: self.回数,
            合計: self.合計,
            最小: self.最小,
            中央値: self.分位点(0.5),
            九十九パーセント点: self.分位点(0.99),
            最大: self.最大,
        }
    }

    // 度数を下から積んで、標本数×割合に達した区分の上端。最大値を越えない。
    fn 分位点(&self, 割合: f64) -> Duration {
        let 必要な度数 = (f64::from(self.回数) * 割合).ceil().max(1.0);
        let mut 積んだ度数 = 0.0;
        for (添字, 度数) in self.度数.iter().enumerate() {
            積んだ度数 += f64::from(*度数);
            if 積んだ度数 >= 必要な度数 {
                let 区分の上端 = Duration::from_micros(u64::try_from(添字 + 1).unwrap_or(u64::MAX).saturating_mul(区分の幅マイクロ秒));
                return 区分の上端.min(self.最大);
            }
        }
        self.最大
    }
}
