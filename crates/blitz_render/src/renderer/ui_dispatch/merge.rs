//! UIメッシュ列を1本の頂点/インデックス列へ結合し、メッシュごとの描画項目
//! (要素ずらし量・ディスクリプタセット・シザー矩形)を組み立てる。

use ash::vk;

use super::super::レンダラー;
use crate::ui_mesh::UIメッシュ;
use crate::ui_scissor::UIシザー矩形px;
use crate::ui_vertex::UI頂点;
use crate::vulkan;

impl レンダラー {
    #[allow(clippy::type_complexity)]
    pub(super) fn メッシュ列を結合する(
        &self,
        メッシュ一覧: &[&UIメッシュ],
        寸法: vk::Extent2D,
    ) -> (Vec<UI頂点>, Vec<u32>, Vec<vulkan::frame::UI描画項目>) {
        let mut 頂点一覧結合 = Vec::new();
        let mut インデックス一覧結合 = Vec::new();
        let mut 項目一覧 = Vec::new();

        for メッシュ in メッシュ一覧 {
            let 頂点要素ずらし量 = i32::try_from(頂点一覧結合.len()).unwrap_or_else(|_| panic!("UI頂点ずらし量がi32に収まらない"));
            let インデックス要素ずらし量 =
                u32::try_from(インデックス一覧結合.len()).unwrap_or_else(|_| panic!("UIインデックスずらし量がu32に収まらない"));
            let インデックス数 = u32::try_from(メッシュ.インデックス一覧.len()).unwrap_or_else(|_| panic!("UIインデックス数がu32に収まらない"));
            項目一覧.push(vulkan::frame::UI描画項目 {
                頂点要素ずらし量,
                インデックス要素ずらし量,
                インデックス数,
                ディスクリプタセット: self.ui一式.setを取得する(メッシュ.テクスチャid),
                シザー: シザーを組み立てる(メッシュ.シザー矩形px, 寸法),
            });
            頂点一覧結合.extend_from_slice(&メッシュ.頂点一覧);
            インデックス一覧結合.extend_from_slice(&メッシュ.インデックス一覧);
        }
        (頂点一覧結合, インデックス一覧結合, 項目一覧)
    }
}

pub(super) fn 有効なメッシュか(メッシュ: &UIメッシュ) -> bool {
    !メッシュ.頂点一覧.is_empty() && !メッシュ.インデックス一覧.is_empty()
}

/// シザー矩形をスワップチェーン寸法へクランプする(egui側の丸め誤差でわずかに
/// はみ出す可能性への防御。範囲外座標はVulkanの検証エラーになるため)。
fn シザーを組み立てる(矩形: UIシザー矩形px, 寸法: vk::Extent2D) -> vk::Rect2D {
    let x = 矩形.x().min(寸法.width);
    let y = 矩形.y().min(寸法.height);
    let 右端 = 矩形.x().saturating_add(矩形.幅()).min(寸法.width);
    let 下端 = 矩形.y().saturating_add(矩形.高さ()).min(寸法.height);
    vk::Rect2D {
        offset: vk::Offset2D {
            x: 非負変換する(x),
            y: 非負変換する(y),
        },
        extent: vk::Extent2D {
            width: 右端.saturating_sub(x),
            height: 下端.saturating_sub(y),
        },
    }
}

/// スワップチェーン寸法由来のu32(実運用でi32::MAXに到達しない)をi32へ変換する。
fn 非負変換する(値: u32) -> i32 {
    i32::try_from(値).unwrap_or_else(|_| panic!("シザー座標がi32に収まらない: {値}"))
}
