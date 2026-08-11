//! アプリの起動指定: 1回ぶんの起動でアプリへ渡す、シーンと枚数と選択肢を組み立てる型。
//!
//! 構築をビルダーの形にするのは、条件ごとに足す選択肢の数と種類が違い、しかもシーンと枚数は
//! どの条件でも必ず要るためである。必ず要る2つを構築の口で受け、任意の選択肢だけをメソッドの連鎖で足すと、
//! シーンを決めないまま起動できる半端な状態が作れない。
//!
//! 完成させるメソッドを別に置かないのは、この型自身が完成物であるためである。
//! アセットルートと書き出し先は条件ごとに変わらないため、この型でなく実行環境の側が足す。

use std::process::Command;

/// アプリへ渡すシーンの名前。プロセス境界の綴りであり、綴りの正本はアプリの世界の宣言が持つ。
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct 検収シーン名(&'static str);

impl 検収シーン名 {
    pub const fn 生成する(綴り: &'static str) -> Self {
        Self(綴り)
    }

    fn 綴り(self) -> &'static str {
        self.0
    }
}

/// 描き切る枚数。読み戻すのは最後の1枚であり、それまでの枚数がストリーミングと時間再構成を落ち着かせる。
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct 描画フレーム数(u32);

impl 描画フレーム数 {
    pub const fn 生成する(枚数: u32) -> Self {
        Self(枚数)
    }
}

pub struct アプリの起動指定 {
    シーン名: 検収シーン名,
    フレーム数: 描画フレーム数,
    追加の選択肢: Vec<String>,
}

impl アプリの起動指定 {
    pub fn シーンと枚数を決める(シーン名: 検収シーン名, フレーム数: 描画フレーム数) -> Self {
        Self {
            シーン名,
            フレーム数,
            追加の選択肢: Vec::new(),
        }
    }

    /// 値を持たない選択肢を1つ足す。
    pub fn 選択肢を足す(mut self, 綴り: &str) -> Self {
        self.追加の選択肢.push(綴り.to_string());
        self
    }

    /// 値を持たない選択肢をまとめて足す。条件ごとの並びを1つの配列で持つ入口のための口である。
    pub fn 選択肢をまとめて足す(mut self, 綴り一覧: &[&str]) -> Self {
        self.追加の選択肢.extend(綴り一覧.iter().map(|綴り| (*綴り).to_string()));
        self
    }

    pub(super) fn コマンドへ並べる(&self, コマンド: &mut Command) {
        コマンド
            .args(["--scene", self.シーン名.綴り()])
            .args(["--frames", &self.フレーム数.0.to_string()])
            .args(&self.追加の選択肢);
    }
}
