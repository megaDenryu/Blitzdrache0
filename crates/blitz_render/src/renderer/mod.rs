//! blitz_render の公開ファサード。Vulkanの全リソースをここに集約し、生成から破棄までのライフサイクルを一元管理する。参照: CLAUDE.md「unsafe の規律」「封じ込め」。ash型は一切ここから公開しない。

mod aerial_composite_input;
mod atmosphere_lut_write;
mod cascade_uniform_write;
mod chunk_bundle;
mod cloth_write;
mod cpu_timing;
mod destroy;
mod draw_dispatch;
mod draw_execute;
mod draw_issue_breakdown;
mod draw_stage_resources;
mod frame_dispatch_inputs;
mod frame_progress;
mod generate;
mod injection;
mod lighting_query_write;
mod measurement_control;
mod optional_frame_inputs;
mod pass_tally;
mod present_resources;
mod presentation;
mod queries;
mod readback_buffer;
mod reconstruct;
mod replace_scene;
mod replace_shader;
mod scene_draw_resources;
mod skin_write;
mod sky_uniform_write;
mod ui_dispatch;
mod ui_texture;
mod uniform_write;
mod view_uniform_write;

use crate::{frame_composition::フレーム構成, validation_counter::検証カウンタ, vulkan};

pub use cpu_timing::CPU区間時間;
pub use draw_issue_breakdown::{パス別描画発行, 描画発行内訳, 段別個体数, 点光源の影の記録内訳, 記録側の計数};
pub use pass_tally::{大気のベイク済み画像生成パス数の記録, 間接照明生成パス数の記録};

/// Vulkanインスタンス・デバイス・スワップチェーン・同期プリミティブを保持し、フレーム構成の順に描いて提示するレンダラー。
/// 前提: `生成する`に渡すハンドルの指すウィンドウは、このレンダラーより長生きすることを呼び出し元が保証する(blitz_appはフィールド宣言順で担保する)。
pub struct レンダラー {
    /// 生成後に変わらないVulkanハンドルの束(ローダー・インスタンス・サーフェス・物理/論理デバイス・キュー)。破棄順序の制約を共有するため1つの型に閉じてある。
    環境: vulkan::gpu_environment::GPU環境,
    /// ウィンドウ寸法に連動して揃って作り直す資源(スワップチェーン・深度バッファ・提示同期)と、その資源が要求寸法に追従しているかどうかの状態の束。深度画像の寸法と提示セマフォの本数がスワップチェーンの決めた値と一致すること、およびゼロ寸法や陳腐化のまま描画へ進めないことはこの型が保つ。
    提示: presentation::提示,
    影の資源: vulkan::shadow_resources::影の資源の組,
    転送環境: vulkan::transfer::転送実行環境,
    /// 描画対象数に連動する資源(描画対象GPU資源・ディスクリプタ・描画入力作業領域)の束。要素数の一致はこの型が保つ。
    シーン描画資源: scene_draw_resources::シーン描画資源,
    /// 索引化した材質テクスチャ表の資源表世代の所有者。set2はこの表が世代ごとに1つ持ち、束は材質IDだけを持つ(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」)。
    材質資源表: vulkan::material_table::材質資源表,
    シェーダー定数: vulkan::uniform::フレームシェーダー定数一式,
    /// scene系パイプライン族が守るセット番号の契約の実物と、描画対象で変わらないset0(フレームスロットごと)とset3。束ごとに複製せずここが1組だけ持ち、束を差し替えても作り直さない。
    セットレイアウト: vulkan::descriptor::シーンセットレイアウト一式,
    共有ディスクリプタ: vulkan::descriptor::共有ディスクリプタセット,
    /// 照明問い合わせのセット(set3)の資源の所有者。進行中フレームスロットごとにヘッダ・方向光レコード列・局所光レコード列とディスクリプタセットを1組ずつ持ち、フェンス通過後のスロットだけを書き換える(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5)。
    照明問い合わせ資源: vulkan::lighting_query::照明問い合わせ資源束,
    /// フレームスロットで参照する資源(コマンドバッファ・描画完了フェンス・取得セマフォ)と巡回状態の束。3つを同じスロットで参照する一致はこの型が保つ。
    フレーム進行: frame_progress::フレーム進行,
    フレーム構成: フレーム構成,
    /// 各描画段階が束縛するパイプラインとレイアウトの束。段階の追加でレンダラー直下のフィールドを増やさないための器。
    描画段階資源: draw_stage_resources::描画段階資源,
    /// 材質を読む描画族(シーン・シャドウ)のパイプライン状態オブジェクトの台帳。パイプラインキーから実体を引く唯一の経路であり、族ごとのパイプラインレイアウトも持つ(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段6)。
    パイプライン台帳: vulkan::pipeline_ledger::材質描画族パイプライン台帳,
    /// スキン付きシーンのときのみ`Some`(判断44)。有無はフレーム描画入力のスキン行列と常に一致させる。
    スキニング: Option<vulkan::skinning::スキニング一式>,
    /// 布付き起動のときのみ`Some`(判断52〜54)。有無はフレーム描画入力の布と常に一致させる。
    布: Option<vulkan::cloth::布一式>,
    /// フレーム構成にポスト処理段階があるときのみ`Some`(判断38・39)。HDR中間画像・光のにじみ・明るさの圧縮の有無をこの1つの`Option`が束ねるため、一部だけが存在する状態をレンダラーからは作れない。
    ポスト処理: Option<vulkan::post_process::ポスト処理一式>,
    局所可視性: vulkan::local_visibility::局所可視性一式,
    /// 時間再構成の方式と、画面寸法に連動する3枚(動きベクトル1枚と履歴2枚)の所有者。方式に依らず常に持つ(判断e)。
    時間再構成: vulkan::temporal_reconstruction::時間再構成一式,
    /// `--particles`指定時のみ`Some`(判断29)。有無でコンピュート更新+粒子描画パスの追加を決める。
    粒子: Option<vulkan::particles::粒子リソース一式>,
    /// タイムスタンプ非対応デバイスでは`None`(判断30: 計測無効は型で表す)。
    gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    cpu区間計測: Option<cpu_timing::CPU区間計測>,
    /// 大気のベイク済み画像を1枚でも焼いたか。上位層の指示が「参照するだけ」でも、まだ焼いていないうちは3枚を焼く指示へ格上げする(見送られたフレームの生成パスはGPUへ届かないため、指示だけでは一度も書かれていない画像を防げない)。
    大気のベイク済み画像を焼いたか: bool,
    /// 生成パスの実行数の記録の束(大気のベイク済み画像と間接照明)。更新判定が働いているかを実測で見る計器であり、常に数える(タイムスタンプの対応に依らずグラフの積み上げから数えるため、計測無効の機材でも値が出る)。
    生成パス数の記録: pass_tally::生成パス数の記録一式,
    実表示計測: vulkan::present_timing::実表示計測,
    /// 開発用UI(egui)描画一式(判断33・34)。表示のオン/オフは入力側の有無で決まるため、常に生成する。
    ui一式: vulkan::ui::UIリソース一式,
    読み戻しバッファ: Option<vulkan::readback::読み戻しバッファ>,
    検証カウンタ: 検証カウンタ,
    /// そのフレームの記録が数えた計器の束。セット番号ごとの束縛回数と、パスごとの描画・パイプライン切替・材質切替を持つ。
    記録の計器: vulkan::frame::記録の計器,
}

impl Drop for レンダラー {
    fn drop(&mut self) {
        self.破棄する();
    }
}
