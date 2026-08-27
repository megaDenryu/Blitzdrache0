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
///
/// `環境`はローダー・インスタンス・サーフェス・物理/論理デバイス・キューを持ち、破棄順序の制約を共有するため1つの型に閉じてある。
/// `提示`はスワップチェーン・深度バッファ・提示同期と、その資源が要求寸法に追従しているかどうかの状態を持つ。
/// 深度画像の寸法と提示セマフォの本数がスワップチェーンの決めた値と一致すること、
/// およびゼロ寸法や陳腐化のまま描画へ進めないことはこの型が保つ。
/// `シーン描画資源`は描画対象GPU資源・ディスクリプタ・描画入力作業領域を持ち、要素数の一致をこの型が保つ。
/// `材質資源表`のset2は世代ごとに1つを表が持ち、束は材質IDだけを持つ(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」)。
/// `セットレイアウト`は描画対象で変わらないset0(フレームスロットごと)とset3もあわせて持つ。束ごとに複製せず1組だけを持ち、束を差し替えても作り直さない。
/// `照明問い合わせ資源`は進行中フレームスロットごとにヘッダ・方向光レコード列・局所光レコード列とディスクリプタセットを1組ずつ持ち、
/// フェンス通過後のスロットだけを書き換える(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5)。
/// `フレーム進行`はコマンドバッファ・描画完了フェンス・取得セマフォを持ち、3つを同じスロットで参照する一致をこの型が保つ。
/// `描画段階資源`は、段階の追加でレンダラー直下のフィールドを増やさないための器である。
/// `パイプライン台帳`が対象にする族はシーンとシャドウであり、パイプラインキーから実体を引く唯一の経路として族ごとのパイプラインレイアウトも持つ
/// (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段6)。
/// `スキニング`と`布`の有無は、フレーム描画入力のスキン行列と布に常に一致させる。
/// `ポスト処理`はHDR中間画像・光のにじみ・明るさの圧縮の有無を1つの`Option`が束ねるため、一部だけが存在する状態をレンダラーからは作れない。
/// `時間再構成`が持つ3枚は動きベクトル1枚と履歴2枚であり、方式に依らず常に持つ(判断e)。
/// `粒子`の有無がコンピュート更新と粒子描画パスの追加を決める。
/// `gpu計測`が`None`になりうるのは、計測が無効であることを型で表すためである(判断30)。
/// `大気のベイク済み画像を焼いたか`が偽のあいだは、上位層の指示が「参照するだけ」でも3枚を焼く指示へ格上げする
/// (見送られたフレームの生成パスはGPUへ届かないため、指示だけでは一度も書かれていない画像を防げない)。
/// `生成パス数の記録`は更新判定が働いているかを実測で見る計器であり、常に数える
/// (タイムスタンプの対応に依らずグラフの積み上げから数えるため、計測無効の機材でも値が出る)。
/// `ui一式`の表示のオン/オフは入力側の有無で決まるため、常に生成する。
/// `記録の計器`はセット番号ごとの束縛回数と、パスごとの描画・パイプライン切替・材質切替を持つ。
pub struct レンダラー {
    gpu環境: vulkan::gpu_environment::GPU環境, // 生成後に変わらないVulkanハンドルの束
    提示: presentation::提示,                  // ウィンドウ寸法に連動して揃って作り直す資源の束
    影の資源: vulkan::shadow_resources::影の資源の組,
    転送環境: vulkan::transfer::転送実行環境,
    シーン描画資源: scene_draw_resources::シーン描画資源, // 描画対象数に連動する資源の束
    材質資源表: vulkan::material_table::材質資源表,       // 索引化した材質テクスチャ表の資源表世代の所有者
    シェーダー定数: vulkan::uniform::フレームシェーダー定数一式,
    セットレイアウト: vulkan::descriptor::シーンセットレイアウト一式, // scene系パイプライン族が守るセット番号の契約の実物
    共有ディスクリプタ: vulkan::descriptor::共有ディスクリプタセット,
    照明問い合わせ資源: vulkan::lighting_query::照明問い合わせ資源束, // 照明問い合わせのセット(set3)の資源の所有者
    フレーム進行: frame_progress::フレーム進行,                       // フレームスロットで参照する資源と巡回状態の束
    フレーム構成: フレーム構成,
    描画段階資源: draw_stage_resources::描画段階資源, // 各描画段階が束縛するパイプラインとレイアウトの束
    パイプライン台帳: vulkan::pipeline_ledger::材質描画族パイプライン台帳, // 材質を読む描画族のパイプライン状態オブジェクトの台帳
    スキニング: Option<vulkan::skinning::スキニング一式>, // スキン付きシーンのときのみ`Some`(判断44)
    布一式: Option<vulkan::cloth::布一式>,            // 布付き起動のときのみ`Some`(判断52〜54)
    ポスト処理: Option<vulkan::post_process::ポスト処理一式>, // フレーム構成にポスト処理段階があるときのみ`Some`(判断38・39)
    局所可視性: vulkan::local_visibility::局所可視性一式,
    時間再構成: vulkan::temporal_reconstruction::時間再構成一式, // 時間再構成の方式と、画面寸法に連動する3枚の所有者
    粒子: Option<vulkan::particles::粒子リソース一式>,           // `--particles`指定時のみ`Some`(判断29)
    gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,          // タイムスタンプ非対応デバイスでは`None`
    cpu区間計測: Option<cpu_timing::CPU区間計測>,
    大気のベイク済み画像を焼いたか: bool,               // 大気のベイク済み画像を1枚でも焼いたか
    生成パス数の記録: pass_tally::生成パス数の記録一式, // 生成パスの実行数の記録の束(大気のベイク済み画像と間接照明)
    実表示計測: vulkan::present_timing::実表示計測,
    ui一式: vulkan::ui::UIリソース一式, // 開発用UI(egui)描画一式(判断33・34)
    読み戻しバッファ: Option<vulkan::readback::読み戻しバッファ>,
    検証カウンタ: 検証カウンタ,
    記録の計器: vulkan::frame::記録の計器, // そのフレームの記録が数えた計器の束
}

impl Drop for レンダラー {
    fn drop(&mut self) {
        self.破棄する();
    }
}
