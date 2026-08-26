//! コンポジションルートが所有する `アプリ`(ApplicationHandler実装)。ウィンドウ生成・レンダラー生成・1フレーム実行の配線だけを行い、ロジックは書かない。
mod animation_state;
mod aspect;
mod cloth_frame;
mod cloth_setup;
mod create;
mod draw_dispatch;
mod exit_report;
pub(crate) mod frame;
mod frame_dump;
mod frame_timing;
mod frame_ui;
mod handler;
mod hot_reload_apply;
mod hot_reload_asset_apply;
mod lod_probe;
mod measurement_setup;
mod particle_setup;
mod persistent_bundles;
mod primitive_draw_item_registry;
mod queries;
mod report_requests;
mod scene_camera;
mod scene_lighting;
pub(crate) mod scene_load;
mod scene_read_count;
mod section_timing;
mod sph_setup;
mod streaming;
pub(crate) mod time_of_day;
mod visibility;
mod window_setup;
use crate::cli::{布モード, 描画対象の並べ方, 空中遠近合成指定, 粒子表示モード, 起動モード};
use crate::{error::起動エラー, hot_reload::ホットリローダー, input::入力状態, overlay_ui::画面へ重ねるUI};
use blitz_render::{クリアカラー, レンダラー};
pub(crate) use frame_timing::{フレーム時間統計, 集計する};
pub(crate) use time_of_day::{太陽天頂区間の記録, 空の再現条件, 遠方環境の鍵の記録, 遠方環境更新判定};
use winit::window::Window;
pub(crate) use {draw_dispatch::時間再構成の突き合わせの要約, streaming::ストリーミング要約};

/// 前提: `レンダラー`フィールドは`window`より前に宣言する。Rustは構造体フィールドを宣言順にDropするため、この順序がレンダラー破棄(surface等)をウィンドウ破棄より必ず先に行うことを保証する(レンダラーの生成前提を満たす)。
///
/// `大域ずらし量`は、カメラ・照明の大域位置と、チャンク座標から導出した描画の基準原点の全部に同じ値を足す。
/// `時間再構成の観測`は、前のフレームの再構成結果を1枚だけ持つ。
/// `天空`は世界の空方針・ゲーム時計・シーンの基準ライティング・そのフレームのライティングと空入力を1つで持つ。
/// `空中遠近合成`で空パスのシェーダーの選択が決まり、実行中は変わらない。
/// `露出`(判断39)と`ブレンド`(判断45)は、CLIの初期値を開発用UIのスライダーが実行中に書き換える。
/// `掴み中だった`は掴み操作のエッジ検出に使い、離した最初のフレームで「離す」介入を発行する。
/// `ストリーミング`の中に、チャンク格子・目録・予算・台帳・読込器はすべて入っている。
/// `可視判定`は束の可視材料・個体別の段の記憶・毎フレームの可視ID列を持つ。ストリーミングを使わない起動時シーンでも要るため、ストリーミング配線の中ではなくアプリが直に持つ。
/// `プリミティブ描画項目台帳`は可視判定の台帳と同じ束IDで対になり、束の追加と解除で一緒に出入りする。
/// `個体詳細段探査`は、段の境界をまたぐ往復を決定的に作るためにカメラを前後させる。
/// `シーン読込計数`は、段の選択や可視判定がディスクI/Oを起こさないことを示す。
/// `スモーク実行`は、自己操作の計画と書き換えの依存を1つで持つ。
pub(crate) struct アプリ {
    レンダラー: Option<レンダラー>,
    window: Option<Window>,
    起動モード: 起動モード,
    シーン: crate::cli::起動時シーン,
    アセットの置き場: crate::runtime_assets::実行時アセットの置き場,
    大域ずらし量: blitz_math::大域ワールド位置, // `--global-offset`で世界全体へ加える平行移動
    描画対象の並べ方: 描画対象の並べ方,
    ホットリローダー: ホットリローダー,
    カメラ: blitz_engine::カメラ,
    入力状態: 入力状態,
    ゲーム配線: crate::game::ゲーム配線,
    現在フレーム: u32,
    視点の履歴: frame::視点の履歴,
    時間再構成の観測: draw_dispatch::時間再構成の観測, // `--report-temporal-reconstruction`指定の実行だけが使う観測の材料
    クリア色: クリアカラー,
    天空: time_of_day::天空配線, // 空と時刻の配線
    世界の描画構成: create::世界の描画構成,
    空中遠近合成: 空中遠近合成指定, // 空の放射輝度の評価方式
    粒子表示: 粒子表示モード,
    報告要求: report_requests::報告要求, // 終了時に出す報告の要求。
    フレーム間隔計測: Option<frame_timing::フレーム間隔計測>,
    画面へ重ねるui: Option<画面へ重ねるUI>,
    開発ui初期有効: bool,
    計測つまみ: frame::描画の計測つまみ,
    フレームダンプ先: crate::cli::フレームダンプ指定,
    読み戻し検収: crate::cli::読み戻し検収起動設定,
    露出: crate::cli::露出倍率,
    ブレンド: crate::cli::アニメーションのブレンド係数,
    アニメーション: Option<animation_state::アニメーション再生>,
    布モード: 布モード,
    布プリセット: Option<cloth_setup::布プリセット>,
    掴み中だった: bool,         // 掴み操作のエッジ検出用
    アニメ時刻: blitz_math::秒, // アニメーション時刻(毎フレーム1/60秒の固定歩進=判断47)
    スモーク基準画像: Option<blitz_render::読み戻し画像>,
    ストリーミング: Option<streaming::ストリーミング配線>, // `--streaming`指定時だけ`Some`
    可視判定: visibility::可視判定配線,                    // インスタンス群の可視判定と個体別LOD
    プリミティブ描画項目台帳: primitive_draw_item_registry::プリミティブ描画項目台帳, // 束ごとのプリミティブ描画項目と、それを詰め直す受け皿
    永続束: persistent_bundles::永続束の状態,
    可視個体の選別の計測: Option<section_timing::区間計測>, // 指定時だけ1フレーム分の走査時間を貯める。
    個体詳細段探査: Option<lod_probe::個体詳細段探査>,      // `--lod-probe-step`指定時だけ`Some`
    シーン読込計数: scene_read_count::シーン読込計数,       // ディスクから実行時シーンを読んだ回数
    スモーク実行: Option<crate::smoke::スモーク実行>,       // `--frames`で起動したときだけ`Some`
    地表の層のタイル: scene_load::地表の層のタイル一式,     // カタログを読むまで決まらず、レンダラーと同じく起動の途中で据わる。
    起動時エラー: Option<起動エラー>,
}
