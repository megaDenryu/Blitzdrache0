//! コマンド送信と提示へ渡す同期入力をまとめる。

use ash::vk;

use super::{
    UI描画入力, ジオメトリ入力, スキニング描画入力, 光のにじみ描画入力, 共有セット束縛, 布描画入力, 明るさの圧縮描画入力, 点光源の影の描画発行,
    点光源の影の束縛, 空中遠近合成描画入力, 空描画入力, 粒子描画入力, 距離区分別のシャドウ入力,
};
use crate::vulkan::atmosphere_lut::大気のベイク済み画像の描画入力;
use crate::vulkan::auto_exposure::自動露出描画入力;
use crate::vulkan::cluster_light_assignment::クラスタ選別の描画入力;
use crate::vulkan::depth_injection::合成深度の注入入力;
use crate::vulkan::indirect_lighting::間接照明の描画入力;
use crate::vulkan::local_visibility::局所可視性描画入力;
use crate::vulkan::point_light_shadow_plan::点光源の影の描画計画;
use crate::vulkan::swapchain::スワップチェーン画像添字;
use crate::vulkan::temporal_reconstruction::{合成入力の注入入力, 時間再構成描画入力};

pub(crate) struct 提示先<'a> {
    pub(crate) loader: &'a ash::khr::swapchain::Device,
    pub(crate) swapchain: vk::SwapchainKHR,
    pub(crate) 画像添字: スワップチェーン画像添字,
    /// 実表示時刻を計測しているときだけ`Some`。提示に付ける単調増加のID。
    pub(crate) 提示id: Option<u64>,
}

#[derive(Clone, Copy)]
pub(crate) struct 描画対象入力<'a> {
    pub(crate) ジオメトリ: &'a [ジオメトリ入力],
    pub(crate) 距離区分別のシャドウ: 距離区分別のシャドウ入力<'a>,
    /// 距離区分のパスへ布を積むかの判断もこの指定が持つ。布は可視個体の選別を通らないため、作業領域の側では外せない。
    pub(crate) 影のキャスター: crate::frame_input::影のキャスター指定,
    /// 描画発行で変わらないset0とset3。シーンパスとシャドウパスの各局面が自分のパイプラインレイアウトで束縛する。
    pub(crate) 共有: 共有セット束縛<'a>,
    /// 起動時の計測条件が選んだ深度プリパスの方式。パイプライン台帳が唯一の持ち主であり、フレームの組み立てはここから読む。
    pub(crate) 深度プリパス方式: crate::frame_composition::深度プリパス方式,
    /// クラスタの選別が積むパスの束縛先と即時定数。全世界がこの経路を通るため`Option`にしない。
    pub(crate) クラスタ選別: クラスタ選別の描画入力,
    /// 点光源の影の面ごとの記録の発行。灯を1件も持たないフレームでは空である。
    pub(crate) 点光源の影の発行: &'a [点光源の影の描画発行],
    /// そのフレームに影を落とす灯の並び。面ごとのパスの本数はこの件数の6倍である。
    pub(crate) 点光源の影の計画: 点光源の影の描画計画,
    pub(crate) 点光源の影の束縛: 点光源の影の束縛,
}

#[derive(Clone, Copy)]
pub(crate) struct 任意描画入力<'a> {
    /// 大気のベイク済み画像を焼き直すフレームだけ`Some`。焼き直さないフレームは生成パスを1本も積まない。
    pub(crate) 大気のベイク済み画像: Option<&'a 大気のベイク済み画像の描画入力>,
    /// 照明問い合わせ契約が遠方環境の枝のフレームだけ`Some`。定数近似の契約では資源そのものが無い。
    pub(crate) 間接照明: Option<&'a 間接照明の描画入力>,
    pub(crate) スキニング: Option<&'a スキニング描画入力>,
    pub(crate) 布: Option<&'a 布描画入力>,
    pub(crate) 空: Option<&'a 空描画入力>,
    /// 大気のベイク済み画像方式で合成を切っていないフレームだけ`Some`。合成パスを空パスの前に1本積む。
    pub(crate) 空中遠近合成: Option<&'a 空中遠近合成描画入力>,
    pub(crate) 粒子: Option<&'a 粒子描画入力>,
    pub(crate) 光のにじみ: Option<&'a 光のにじみ描画入力>,
    pub(crate) 明るさの圧縮: Option<&'a 明るさの圧縮描画入力>,
    /// 露出方式がヒストグラム自動の世界だけ`Some`。時刻別固定の世界では集計も更新も1本も積まない。
    pub(crate) 自動露出: Option<&'a 自動露出描画入力>,
    /// 拡散間接方式が局所可視性補正付き環境の世界だけ`Some`。環境のみの世界では遮蔽の標本化もぼかしも1本も積まない。
    pub(crate) 局所可視性: Option<&'a 局所可視性描画入力>,
    /// 時間再構成方式が履歴混合で、かつポスト処理を組む構成のフレームだけ`Some`。使わないの世界では再構成のパスを
    /// 1本も積まず、シーンの色はHDR中間画像へ直に描かれる。
    pub(crate) 時間再構成: Option<&'a 時間再構成描画入力>,
    /// 検収が時間再構成の合成入力を据えた実行だけ`Some`。本番の起動では転送パスを1本も積まない。
    pub(crate) 時間再構成の合成入力: Option<合成入力の注入入力>,
    /// 検収が合成深度を据えた実行だけ`Some`。本番の起動では転送パスを1本も積まない。
    pub(crate) 合成深度の注入: Option<合成深度の注入入力>,
    pub(crate) ui: Option<&'a UI描画入力>,
}

pub(crate) struct 同期入力 {
    pub(crate) 取得セマフォ: vk::Semaphore,
    pub(crate) 提示セマフォ: vk::Semaphore,
    pub(crate) 描画完了フェンス: vk::Fence,
}
