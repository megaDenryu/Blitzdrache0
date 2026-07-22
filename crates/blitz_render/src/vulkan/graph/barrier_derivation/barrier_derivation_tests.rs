//! バリア導出の純粋部分の単体テスト。
//! 参照: `_doc/設計/レンダーグラフ.md`「同期導出の規則」。
#![allow(clippy::unwrap_used, clippy::expect_used)]

mod difference_tests;

use std::collections::HashMap;

use super::{導出する, 遷移地点};
use crate::vulkan::graph::handle::画像ハンドル;
use crate::vulkan::graph::initial_state::{前フレーム深度書き込み直後状態, 取得直後の色画像状態};
use crate::vulkan::graph::pass_resource_usage::パスリソース使用;
use crate::vulkan::graph::usage::画像用途;

fn 空のパス(名前: &'static str) -> パスリソース使用 {
    パスリソース使用 {
        名前,
        読み画像: Vec::new(),
        書き画像: Vec::new(),
        読みバッファ: Vec::new(),
        書きバッファ: Vec::new(),
    }
}

#[test]
fn シーン描画から読み戻しコピー提示までの3パス列で期待通りのバリアが出る() {
    let カラー = 画像ハンドル::生成する(0, 0);
    let 深度 = 画像ハンドル::生成する(1, 0);

    let mut 初期状態 = HashMap::new();
    初期状態.insert(カラー, 取得直後の色画像状態());
    初期状態.insert(深度, 前フレーム深度書き込み直後状態());

    let パス列 = vec![
        パスリソース使用 {
            書き画像: vec![(カラー, 画像用途::カラー出力), (深度, 画像用途::深度出力)],
            ..空のパス("シーン描画")
        },
        パスリソース使用 {
            読み画像: vec![(カラー, 画像用途::転送元)],
            ..空のパス("読み戻しコピー")
        },
    ];
    let 最終用途 = [(カラー, 画像用途::提示)];

    let 結果 = 導出する(&初期状態, &パス列, &最終用途);

    assert_eq!(結果.len(), 3, "パス数2 + グラフ終端で3地点になるはず");

    let シーン描画前 = &結果[0];
    assert_eq!(シーン描画前.地点, 遷移地点::パスの前 { 名前: "シーン描画" });
    assert_eq!(シーン描画前.バリア一覧.len(), 2, "カラー・深度の両方が初回遷移するはず");
    let カラー遷移 = シーン描画前
        .バリア一覧
        .iter()
        .find(|バリア| バリア.ハンドル == カラー)
        .expect("カラーの遷移が見つからない");
    assert_eq!(カラー遷移.前.layout, ash::vk::ImageLayout::UNDEFINED, "初回はUNDEFINEDから開始する");
    assert_eq!(カラー遷移.今.layout, ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let コピー前 = &結果[1];
    assert_eq!(
        コピー前.地点,
        遷移地点::パスの前 {
            名前: "読み戻しコピー"
        }
    );
    assert_eq!(コピー前.バリア一覧.len(), 1, "コピー前はカラーのみ書き→読みで遷移する");
    assert_eq!(コピー前.バリア一覧[0].ハンドル, カラー);
    assert_eq!(コピー前.バリア一覧[0].今.layout, ash::vk::ImageLayout::TRANSFER_SRC_OPTIMAL);

    let 終端 = &結果[2];
    assert_eq!(終端.地点, 遷移地点::グラフ終端);
    assert_eq!(終端.バリア一覧.len(), 1, "終端はカラーのみ提示へ遷移する(深度は提示しない)");
    assert_eq!(終端.バリア一覧[0].今.layout, ash::vk::ImageLayout::PRESENT_SRC_KHR);
}
