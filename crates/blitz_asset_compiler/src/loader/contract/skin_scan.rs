//! スキンとアニメーションの検査。見るのは先頭メッシュのスキンの有無と、そこから決まるアニメーションの扱いである。
//! 由来は`loader::mesh::skin_vertex`がスキン付きプリミティブにJOINTS_0とWEIGHTS_0の両方を要求すること、
//! `loader::animation`がCUBICSPLINE補間を拒むこと、および`loader::mod`がスキンの無いメッシュではアニメーション一覧を空にすることである。

use super::inspection::開いた文書の契約検査;
use super::target::{名前を写す, 対象位置};

/// 先頭メッシュがスキンを持つか。アニメーションの宣言が違反になるか警告にもならないかがこの枝で逆になるため、真偽値でなく枝で表す。
#[derive(Clone, Copy, PartialEq, Eq)]
enum 先頭メッシュのスキン {
    ある,
    ない,
}

impl 開いた文書の契約検査<'_> {
    pub(super) fn スキンとアニメーションを検査する(&mut self) {
        let スキン = self.先頭メッシュのスキンの有無を調べる();
        if スキン == 先頭メッシュのスキン::ある {
            self.スキン付きプリミティブの頂点属性を検査する();
        }
        self.アニメーションの宣言を検査する(スキン);
    }

    fn 先頭メッシュのスキンの有無を調べる(&self) -> 先頭メッシュのスキン {
        let ある = self
            .文書の全体()
            .nodes()
            .any(|ノード| ノード.mesh().is_some_and(|メッシュ| メッシュ.index() == 0) && ノード.skin().is_some());
        if ある {
            先頭メッシュのスキン::ある
        } else {
            先頭メッシュのスキン::ない
        }
    }

    fn スキン付きプリミティブの頂点属性を検査する(&mut self) {
        let Some(メッシュ) = self.文書の全体().meshes().next() else {
            return;
        };
        for (添字, プリミティブ) in メッシュ.primitives().enumerate() {
            let 関節がある = プリミティブ.get(&gltf::Semantic::Joints(0)).is_some();
            let 重みがある = プリミティブ.get(&gltf::Semantic::Weights(0)).is_some();
            if 関節がある && 重みがある {
                continue;
            }
            self.違反を記す(
                対象位置::プリミティブ {
                    メッシュ添字: 0, 添字
                },
                "スキン付きのメッシュにJOINTS_0とWEIGHTS_0が揃っていない。ローダーは両方を要求する",
                "Blenderで全頂点をボーンへ重み付けしてから、書き出し設定でスキンを含める",
            );
        }
    }

    fn アニメーションの宣言を検査する(&mut self, スキン: 先頭メッシュのスキン) {
        for (添字, アニメーション) in self.文書の全体().animations().enumerate() {
            let 位置 = || 対象位置::アニメーション {
                添字,
                名前: 名前を写す(アニメーション.name()),
            };
            if スキン == 先頭メッシュのスキン::ない {
                self.違反を記す(
                    位置(),
                    "スキンの無いメッシュにアニメーションが宣言されている。ローダーはスキンが無いとアニメーション一覧を空にするため、この動きは絵に出ない",
                    "ボーンとスキンを付けてジョイントの動きとして書き出す。オブジェクト自身の移動・回転はエンジン側のシーン定義で与える",
                );
                continue;
            }
            if アニメーション
                .channels()
                .any(|チャンネル| チャンネル.sampler().interpolation() == gltf::animation::Interpolation::CubicSpline)
            {
                self.違反を記す(
                    位置(),
                    "CUBICSPLINE補間のチャンネルを含む。ローダーはCUBICSPLINEを未対応として拒む",
                    "Blenderでキーフレームの補間を線形かステップにしてから書き出す",
                );
            }
        }
    }
}
