//! 升目の側面のはめ口へ何を入れるかの素データと、その素データを領域の宣言へ解く写し。
//!
//! 素データを領域の型(`blitz_assembly`の`はめ口の値`)と分けるのは、接する相手が違うためである。素データは
//! JSONの直列化へ接して欄の名前と綴りを持ち、領域の型は接合点の宣言へ接して指示を組む。
//!
//! 「無し」を欄の不在で表さず`入れない`という枝で表すのは、壁を消した保存物と壁の欄を書き忘れた保存物を
//! 読む側が区別できるようにするためである。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断8」

use blitz_assembly::{はめ口の値, 壁の外面へ付ける飾り, 壁の種類, 煙突の段数};
use serde::{Deserialize, Serialize};

use super::error::建物の格子のソースエラー;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum 壁の種類ソース {
    平壁,
    窓壁,
    扉枠付きの壁,
}

impl 壁の種類ソース {
    fn 領域の宣言へ解く(self) -> 壁の種類 {
        match self {
            Self::平壁 => 壁の種類::平壁,
            Self::窓壁 => 壁の種類::窓壁,
            Self::扉枠付きの壁 => 壁の種類::扉枠付きの壁,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "種類", content = "値")]
pub enum 壁の外面の飾りソース {
    付けない,
    出窓を差し込む,
    煙突を立てる { 段数: u32 },
}

impl 壁の外面の飾りソース {
    fn 領域の宣言へ解く(self, 建物定義: &str) -> Result<壁の外面へ付ける飾り, 建物の格子のソースエラー> {
        match self {
            Self::付けない => Ok(壁の外面へ付ける飾り::付けない),
            Self::出窓を差し込む => Ok(壁の外面へ付ける飾り::出窓を差し込む),
            Self::煙突を立てる { 段数 } => {
                let 段数 = usize::try_from(段数).map_err(|原因| 欄が不正なエラーを作る(建物定義, "煙突の段数", &原因.to_string()))?;
                Ok(壁の外面へ付ける飾り::煙突を立てる {
                    段数: 煙突の段数::生成する(段数)
                        .map_err(|原因| 欄が不正なエラーを作る(建物定義, "煙突の段数", &原因.to_string()))?,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "種類", content = "値")]
pub enum はめ口ソース {
    入れない,
    壁を入れる {
        壁の種類: 壁の種類ソース,
        外面の飾り: 壁の外面の飾りソース,
    },
}

impl はめ口ソース {
    pub(super) fn 領域の宣言へ解く(self, 建物定義: &str) -> Result<はめ口の値, 建物の格子のソースエラー> {
        match self {
            Self::入れない => Ok(はめ口の値::入れない),
            Self::壁を入れる {
                壁の種類, 外面の飾り
            } => Ok(はめ口の値::壁を入れる {
                種類: 壁の種類.領域の宣言へ解く(),
                外面の飾り: 外面の飾り.領域の宣言へ解く(建物定義)?,
            }),
        }
    }
}

fn 欄が不正なエラーを作る(建物定義: &str, 欄: &'static str, 説明: &str) -> 建物の格子のソースエラー {
    建物の格子のソースエラー::欄が不正 {
        建物定義: 建物定義.to_string(),
        欄,
        説明: 説明.to_string(),
    }
}
