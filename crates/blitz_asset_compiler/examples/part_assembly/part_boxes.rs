//! 据えた順の部品と、その部品を配置で写した箱の対応。担当するのは、展開の結果と読み取った境界箱を突き合わせて
//! 「何番目に据えた部品の、群ローカルでの箱」を答えることである。
//!
//! 据えた順で持つのは、手順の指示が親を据えた番号で指すためである。部品の種類ごとにまとめた眺めからは、
//! 同じ部品が複数据わったときにどれが何番目かを取り出せない。

use std::collections::BTreeMap;

use blitz_assembly::{群ローカルの箱, 部品ごとの配置表, 部品の境界箱};

pub struct 部品ごとの箱 {
    据えた順の部品の識別子: Vec<String>,
    据えた順の箱: Vec<Option<群ローカルの箱>>,
}

impl 部品ごとの箱 {
    pub fn 配置表と箱の表から作る(配置表: &部品ごとの配置表, 箱の表: &BTreeMap<String, 部品の境界箱>) -> Self {
        let mut 据えた順の部品の識別子 = Vec::with_capacity(配置表.据えた順().len());
        let mut 据えた順の箱 = Vec::with_capacity(配置表.据えた順().len());
        for 据えた in 配置表.据えた順() {
            let 文字列 = 据えた.識別子().文字列().to_string();
            据えた順の箱.push(箱の表.get(&文字列).map(|箱| 箱.配置で写す(&据えた.配置())));
            据えた順の部品の識別子.push(文字列);
        }
        Self {
            据えた順の部品の識別子, 据えた順の箱
        }
    }

    pub fn 番号の部品の識別子(&self, 番号: usize) -> String {
        self.据えた順の部品の識別子.get(番号).cloned().unwrap_or_else(|| "(据わっていない)".to_string())
    }

    pub fn 番号に対応する箱を参照する(&self, 番号: usize) -> Option<群ローカルの箱> {
        self.据えた順の箱.get(番号).copied().flatten()
    }
}
