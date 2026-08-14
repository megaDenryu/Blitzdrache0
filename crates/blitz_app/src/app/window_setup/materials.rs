//! resumed時の構築が受け取る材料と、返す一式の名前つきの形。担当するのは、17個の引数として並んでいた値を
//! 「何を読むか」と「どう描くか」の2つの束へ分け、返す6つの値へ名前を与えることだけである。
//!
//! 位置で並べた引数とタプルの戻り値をやめるのは、どちらも取り違えを型が止められないためである。引数側は同じ
//! `bool`が3つ並び、戻り値側は`Option`が2つ並んでいて、順序を入れ替えてもコンパイルが通っていた。

use blitz_render::レンダラー;
use winit::window::Window;

use super::super::animation_state::アニメーション再生;
use super::super::cloth_setup::布プリセット;
use super::super::scene_load;
use crate::cli::{布モード, 描画対象の並べ方, 空中遠近合成指定, 粒子表示モード, 起動時シーン};
use crate::overlay_ui::画面へ重ねるUI;
use crate::runtime_assets::実行時アセットの置き場;

/// 起動時に読む世界の材料。どの生成物をどこから読み、どんな並びで描画対象を作るかを決める。
pub(in crate::app) struct 読み込む世界の材料<'材料> {
    pub(in crate::app) シーン: &'材料 起動時シーン,
    pub(in crate::app) アセットの置き場: &'材料 実行時アセットの置き場,
    pub(in crate::app) 描画対象の並べ方: 描画対象の並べ方,
    /// `--global-offset`が世界全体へ加える平行移動。描画の基準原点へ最後に合成する。
    pub(in crate::app) 大域平行移動: blitz_math::大域ワールド位置,
    pub(in crate::app) 布モード: 布モード,
}

/// 起動時に決まり、以降は変わらない画面の作り方。世界の描画構成・天空配線・計測の要求が決めた値をここへ集める。
pub(in crate::app) struct 画面の作り方 {
    pub(in crate::app) 粒子表示: 粒子表示モード,
    pub(in crate::app) 空中遠近合成: 空中遠近合成指定,
    pub(in crate::app) 開発ui初期有効: bool,
    pub(in crate::app) フレーム構成: blitz_render::フレーム構成,
    pub(in crate::app) 実表示計測要求: blitz_render::実表示計測要求,
    pub(in crate::app) 影の一辺解像度: blitz_render::cascade::影の一辺解像度,
    pub(in crate::app) 照明問い合わせ契約: blitz_render::indirect_lighting::照明問い合わせ契約,
    pub(in crate::app) 自動露出の設定: blitz_render::auto_exposure::自動露出の設定,
    pub(in crate::app) 局所可視性の描画設定: blitz_render::local_visibility::局所可視性の描画設定,
    pub(in crate::app) 時間再構成の描画設定: blitz_render::temporal_reconstruction::時間再構成の描画設定,
}

/// resumedで組み上げた一式。呼び出し元がアプリのフィールドへそのまま移す。
///
/// 前提: レンダラーがウィンドウより先に破棄されることは`アプリ`のフィールド宣言順が保証する。この型は名前で
/// 受け渡すだけであり、フィールドの並びに意味を持たせない。
pub(in crate::app) struct 起動時に組み上げた一式 {
    pub(in crate::app) window: Window,
    pub(in crate::app) レンダラー: レンダラー,
    pub(in crate::app) 画面へ重ねるui: 画面へ重ねるUI,
    pub(in crate::app) アニメーション: Option<アニメーション再生>,
    pub(in crate::app) 布プリセット: Option<布プリセット>,
    pub(in crate::app) 登録一式: scene_load::束の登録一式,
    pub(in crate::app) 遠景: Option<(blitz_engine::シーンデータ, scene_load::束の描画入力)>,
}
