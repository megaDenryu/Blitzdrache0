//! 画面を空だけの領域とジオメトリの領域へ分ける工程。受け取るのは領域マスクの読み戻し画像、返すのは画素ごとの区分である。
//!
//! 区分はライティングもポスト処理も空も外した1枚から作る。その条件では地表と植生がベースカラーのまま描かれ、
//! 何も描かれない画素だけがクリア色で残るため、空とジオメトリの境が時刻に依らず1通りに決まる。
//! クリア色は左上の画素から取る。カメラを地平線より上へ向けた構図では、画面左上に地面が来ることはない。

use crate::acceptance::{画素の番号, 読み戻し画像};

/// その画素が空だけの領域か、ジオメトリの領域か。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 区分 {
    空,
    ジオメトリ,
}

pub(super) struct 領域区分 {
    区分一覧: Vec<区分>,
    pub(super) 空画素数: u64,
    pub(super) 幾何画素数: u64,
}

impl 領域区分 {
    pub(super) fn 作る(マスク: &読み戻し画像) -> Result<Self, String> {
        let クリア色 = マスク.番号の画素(画素の番号::生成する(0));
        let mut 区分一覧 = Vec::with_capacity(マスク.画素数());
        let (mut 空画素数, mut 幾何画素数) = (0_u64, 0_u64);
        for 添字 in 0..マスク.画素数() {
            if マスク.番号の画素(画素の番号::生成する(添字)) == クリア色 {
                区分一覧.push(区分::空);
                空画素数 += 1;
            } else {
                区分一覧.push(区分::ジオメトリ);
                幾何画素数 += 1;
            }
        }
        if 空画素数 == 0 || 幾何画素数 == 0 {
            return Err(format!("領域マスクが片側に寄った: 空{空画素数}画素、ジオメトリ{幾何画素数}画素"));
        }
        Ok(Self {
            区分一覧,
            空画素数,
            幾何画素数,
        })
    }

    pub(super) fn 区分(&self, 添字: usize) -> 区分 {
        self.区分一覧[添字]
    }

    pub(super) fn 画素数(&self) -> usize {
        self.区分一覧.len()
    }

    pub(super) fn 寸法が合うか(&self, 絵: &読み戻し画像) -> Result<(), String> {
        if self.画素数() != 絵.画素数() {
            return Err(format!("領域マスクと絵の画素数が違う: {}と{}", self.画素数(), 絵.画素数()));
        }
        Ok(())
    }

    /// 指定した列で、空領域が上から連続する行の範囲(先頭行・末尾行)を求める。空代表画素の照合が、
    /// 天頂寄り(範囲の先頭側)と地平線直上(範囲の末尾側)の2点を選ぶのに使う。
    pub(super) fn 縦の空範囲を求める(&self, 幅: usize, 列: usize) -> Option<(usize, usize)> {
        let 高さ = self.区分一覧.len() / 幅;
        let (mut 先頭, mut 末尾) = (None, None);
        for 行 in 0..高さ {
            if self.区分一覧[行 * 幅 + 列] == 区分::空 {
                先頭.get_or_insert(行);
                末尾 = Some(行);
            } else if 先頭.is_some() {
                break;
            }
        }
        先頭.zip(末尾)
    }
}
