//! 元の一覧の原文を、見出しの行ごとの塊へ分ける工程。受け取るのは原文、返すのは塊の並びである。
//! 最初の見出しより前の行は、見出しの無い塊に入る。見出しの行の見分け方は `xtask/src/conform/doc_section.rs` と同じである(1個から6個の井桁の後に空白)。

#[derive(Clone, Copy)]
pub(super) struct 見出しの行<'a> {
    井桁の数: usize,
    名前: &'a str,
    行番号: usize,
}

impl<'a> 見出しの行<'a> {
    fn 読む(文字列: &'a str, 行番号: usize) -> Option<Self> {
        let 井桁の数 = 文字列.chars().take_while(|文字| *文字 == '#').count();
        if 井桁の数 == 0 || 井桁の数 > 6 {
            return None;
        }
        let 残り = &文字列[井桁の数..];
        残り.starts_with(' ').then(|| Self {
            井桁の数, 名前: 残り.trim(), 行番号
        })
    }

    pub(super) fn 井桁の数(&self) -> usize {
        self.井桁の数
    }

    pub(super) fn 名前(&self) -> &'a str {
        self.名前
    }

    pub(super) fn 行番号(&self) -> usize {
        self.行番号
    }
}

#[derive(Clone, Copy)]
pub(super) struct 本文の行<'a> {
    行番号: usize,
    文字列: &'a str,
}

impl<'a> 本文の行<'a> {
    pub(super) fn 行番号(&self) -> usize {
        self.行番号
    }

    pub(super) fn 文字列(&self) -> &'a str {
        self.文字列
    }

    /// 表の行か。行の頭の空白を除いて縦棒で始まる行を表の行とみなす。
    pub(super) fn 表の行か(&self) -> bool {
        self.文字列.trim_start().starts_with('|')
    }
}

pub(super) struct 節の塊<'a> {
    見出し: Option<見出しの行<'a>>,
    本文: Vec<本文の行<'a>>,
}

impl<'a> 節の塊<'a> {
    pub(super) fn 原文を分ける(原文: &'a str) -> Vec<Self> {
        let mut 塊一覧 = vec![Self { 見出し: None, 本文: Vec::new() }];
        for (添字, 文字列) in 原文.lines().enumerate() {
            let 行番号 = 添字 + 1;
            match 見出しの行::読む(文字列, 行番号) {
                Some(見出し) => 塊一覧.push(Self {
                    見出し: Some(見出し), 本文: Vec::new()
                }),
                None => {
                    if let Some(最後の塊) = 塊一覧.last_mut() {
                        最後の塊.本文.push(本文の行 { 行番号, 文字列 });
                    }
                }
            }
        }
        塊一覧
    }

    pub(super) fn 見出し(&self) -> Option<見出しの行<'a>> {
        self.見出し
    }

    pub(super) fn 本文(&self) -> &[本文の行<'a>] {
        &self.本文
    }

    /// 本文の行のうち、指定した前置きで始まる箇条書きの値(前置きより後を前後の空白を除いたもの)の並び。
    pub(super) fn 箇条の値一覧(&self, 前置き: &str) -> Vec<&'a str> {
        self.本文.iter().filter_map(|行| 行.文字列.strip_prefix(前置き)).map(str::trim).collect()
    }
}
