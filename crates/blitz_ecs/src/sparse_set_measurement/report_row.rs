//! 疎な集合の計測の1項目ぶんの記録と、その1行の出力。項目・個体数・所要時間・個体1件あたりの所要時間を1行で出す。
//! 素朴な置き場の行と別の型にしているのは、素朴な置き場の計測を触らずに置き場の名前だけを違えるためである。

use std::time::Duration;

pub(crate) struct 疎な集合の計測の結果の行 {
    項目名: &'static str,
    個体数: u32,
    所要時間: Duration,
}

impl 疎な集合の計測の結果の行 {
    pub(crate) fn 生成する(項目名: &'static str, 個体数: u32, 所要時間: Duration) -> Self {
        Self { 項目名, 個体数, 所要時間 }
    }

    pub(crate) fn 結果を1行で出力する(&self) {
        println!(
            "疎な集合 項目={} 個体数={} 所要時間={}マイクロ秒 個体1件あたり={}ナノ秒",
            self.項目名,
            self.個体数,
            self.所要時間.as_micros(),
            self.個体1件あたりの所要時間().as_nanos()
        );
    }

    fn 個体1件あたりの所要時間(&self) -> Duration {
        if self.個体数 == 0 {
            panic!("個体数0の計測の行を作った(計測は必ず1件以上の個体を測る)")
        }
        self.所要時間 / self.個体数
    }
}
