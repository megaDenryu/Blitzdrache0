//! 種からのマップ生成が満たすべき2つの性質の検収。受け取るのは種、返すのは要約である。
//! クリーンな2つの生成がバイト一致することは`determinism`が、同じ種での再実行が1本も焼き直さないことは
//! `incremental`が確かめる。ファイルの畳み込みは`file_digest`、報告の行の読み取りは`tally_line`が持つ。
//!
//! 増分を先に確かめるのは、そこで既定のルートが今の種の生成物へ揃うためである。実行時形式の決定性は
//! 既定のソースルートから2つの出力ルートへ焼いて比べるため、既定のソースが揃っている必要がある。
//! 続く実行の段が既定のルートを開くことも、この順で満たされる。
//! 参照: `_doc/設計/大規模世界の生成と遠景.md`

mod determinism;
mod file_digest;
mod incremental;
mod tally_line;

use std::path::PathBuf;

/// 決定性の検収が使う4つのルート。既定のルートと分けるのは、遊ぶ世界を検収が消して作り直さないためである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 検収に使うルート {
    ソースの一度目,
    ソースの二度目,
    実行時形式の一度目,
    実行時形式の二度目,
}

impl 検収に使うルート {
    fn パス(self) -> PathBuf {
        let 末端 = match self {
            Self::ソースの一度目 => "source_a",
            Self::ソースの二度目 => "source_b",
            Self::実行時形式の一度目 => "runtime_x",
            Self::実行時形式の二度目 => "runtime_y",
        };
        PathBuf::from("target/fox_tour_generation_check").join(末端)
    }
}

/// 突き合わせから除くファイル。生成台帳は生成器の実行ファイルの内容ハッシュと、ルートごとに違うソースのパスから
/// 導いた値を持つため、別のルートへの生成どうしで中身が一致しない。台帳は生成物ではなく増分の途中状態である。
fn 突き合わせから除くファイル名() -> [&'static str; 1] {
    ["generation_ledger.txt"]
}

pub(super) fn 種からの生成を確かめる(種: &str) -> Result<String, String> {
    let 増分 = incremental::同じ種での再実行が焼き直さないことを確かめる(種)?;
    let 決定性 = determinism::同じ種から同じバイト列が出ることを確かめる(種)?;
    Ok(format!("{決定性}、{増分}"))
}
