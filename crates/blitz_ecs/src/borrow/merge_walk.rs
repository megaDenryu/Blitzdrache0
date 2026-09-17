//! 実行時個体IDの昇順の合流走査: IDの昇順に並んだ2つの走査を突き合わせ、両方に現れる個体だけを返す反復子。
//! 2つの置き場の密な列がどちらも実行時個体IDの昇順である(契約25)ことに依り、片方を走査して他方を引く形を採らないのは、
//! 両方を可変に貸す走査を安全なRustの反復子として書けるのがこの形だけだからである。1回の反復でヒープを確保しない。
//!
//! 前提: 2つの走査はどちらも実行時個体IDの昇順に並んでいる。置き場の走査が並びの崩れを panic で止めるため、ここでは確かめない。

use std::cmp::Ordering;
use std::iter::Peekable;

use crate::runtime_entity_id::ゲーム世界の実行時個体ID;

pub(crate) struct 実行時個体IDの昇順の合流走査<甲, 乙>
where
    甲: Iterator,
    乙: Iterator,
{
    甲: Peekable<甲>,
    乙: Peekable<乙>,
}

impl<甲, 乙, 甲の値, 乙の値> 実行時個体IDの昇順の合流走査<甲, 乙>
where
    甲: Iterator<Item = (ゲーム世界の実行時個体ID, 甲の値)>,
    乙: Iterator<Item = (ゲーム世界の実行時個体ID, 乙の値)>,
{
    pub(crate) fn 生成する(甲: 甲, 乙: 乙) -> Self {
        Self { 甲: 甲.peekable(), 乙: 乙.peekable() }
    }
}

impl<甲, 乙, 甲の値, 乙の値> Iterator for 実行時個体IDの昇順の合流走査<甲, 乙>
where
    甲: Iterator<Item = (ゲーム世界の実行時個体ID, 甲の値)>,
    乙: Iterator<Item = (ゲーム世界の実行時個体ID, 乙の値)>,
{
    type Item = (ゲーム世界の実行時個体ID, 甲の値, 乙の値);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let 甲の個体 = self.甲.peek()?.0;
            let 乙の個体 = self.乙.peek()?.0;
            match 甲の個体.cmp(&乙の個体) {
                Ordering::Less => {
                    self.甲.next();
                }
                Ordering::Greater => {
                    self.乙.next();
                }
                Ordering::Equal => {
                    let (個体, 甲の値) = self.甲.next()?;
                    let (_, 乙の値) = self.乙.next()?;
                    return Some((個体, 甲の値, 乙の値));
                }
            }
        }
    }
}
