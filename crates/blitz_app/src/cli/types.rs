//! CLI引数から得る型: 起動モード・起動設定一式。既定値の組み立ては`default`にある。

mod default;
mod frame_dump_setting;
mod launch_mode;
mod readback_verification;
pub(in crate::cli) use frame_dump_setting::走査の書き出し先を確かめる;
pub(crate) use readback_verification::読み戻し検収起動設定;
pub(crate) use {frame_dump_setting::フレームダンプ指定, launch_mode::起動モード};

use super::{シャドウ計測起動設定, ストリーミング起動設定, 布モード, 平行移動起動設定, 遊ぶゲームの指定};
use super::{描画対象の並べ方, 時間帯起動設定, 画面画素位置, 粒子表示モード};
use std::path::PathBuf;

/// CLI引数から得た起動設定一式。
pub(crate) struct 起動設定 {
    pub(crate) モード: 起動モード,
    /// ホットリロードの監視対象となるエントリファイル。既定は`shaders/scene.slang`(存在しなければ監視無効)。
    /// `import`で参照される他の.slangファイル(`pbr.slang`等)はこのエントリファイルと同じディレクトリから解決されるため、個別指定はしない。ディレクトリ全体をmtime走査で監視する。
    pub(crate) シェーダー監視パス: PathBuf,
    /// 表示するシーンのアセットID。既定は`quad`(常に存在し決定的)。
    pub(crate) シーン名: String,
    /// カタログの実行時アセットパスの基準ディレクトリ。既定は`target/runtime_assets`。
    pub(crate) アセットルート: PathBuf,
    /// 起動時シーンの描画対象の並べ方。`--object-count`が複製する件数を、`--reverse-draw-order`が束の中の走査順を決める。
    pub(crate) 描画対象の並べ方: 描画対象の並べ方,
    /// `--unlit`指定でfalse。既定はtrue(PBRライティング有効、判断26)。
    pub(crate) ライティング有効: bool,
    /// 粒子系GPUパスで表示する検証対象。既定はなし。
    pub(crate) 粒子表示: 粒子表示モード,
    /// `--report-gpu-times`指定でtrue。既定はfalse(パス別GPU時間の終了時コンソール出力、判断30)。
    pub(crate) gpu時間報告: bool,
    /// `--report-gpu-frame-times`指定でtrue。既定はfalse(パス別GPU時間の窓へ入る前の生の値をフレーム別に全件出す)。
    /// 分位を独立に計算し直して報告値と突き合わせる検収だけが使う指定であり、行数が測定フレーム数×区間数まで増える。
    pub(crate) gpu時間のフレーム別生値報告: bool,
    /// `--report-atmosphere-passes`指定でtrue。既定はfalse(フレームごとの大気のベイク済み画像生成パス本数を終了時に出力する)。
    pub(crate) 大気のベイク済み画像パス数報告: bool,
    /// `--report-frame-times`指定でtrue。最初の120フレームを除いたCPU側フレーム間隔分布を終了時に出力する。
    pub(crate) フレーム時間報告: bool,
    /// `--report-memory`指定でtrue。Vulkan専用メモリ確保の現在数・上限・用途別量を終了時に出力する。
    pub(crate) gpuメモリ報告: bool,
    /// `--report-draw-issue`指定でtrue。最終フレームのパス別描画発行数・候補数・可視数・個体数を終了時に出力する。
    pub(crate) 描画発行報告: bool,
    /// `--report-sun-angle`指定でtrue。その実行が使った太陽の高度と方位を終了時に出力する。
    pub(crate) 太陽角度報告: bool,
    /// `--report-caster-distance`指定でtrue。最終フレームのキャスター候補をカメラからの距離帯へ振り分けた分布を終了時に出力する。
    pub(crate) キャスター距離分布報告: bool,
    /// `--report-instance-sections`指定でtrue。可視個体の選別の走査時間を測り、レンダラーCPU区間と併せて終了時に出力する。
    /// フレーム時間報告と別の指定にするのは、区間の内訳が要るのは物量計測だけであり、既存の時系列を採る条件へ相乗りさせないためである。
    pub(crate) インスタンス区間報告: bool,
    /// `--no-instance-cull`指定でfalse。既定はtrue。falseは全個体を描くため、可視判定の有無で読み戻し画像を比べる検収が成立する。
    pub(crate) インスタンス可視判定有効: bool,
    /// `--no-instance-lod`指定でfalse。既定はtrue。falseは全個体を最詳細段で描くため、段の選択の有無で読み戻し画像を比べる検収が成立する。
    pub(crate) インスタンス段選択有効: bool,
    /// `--no-instance-shadow`指定でfalse。既定はtrue。falseはどの距離区分も個体を影の候補にしないため、シャドウパスに残る費用が地形と添付処理だけになる計測用の対照が作れる。
    pub(crate) インスタンス影キャスター有効: bool,
    /// `--no-shadow-casters`指定でfalse。既定はtrue。falseは地形も個体も布も距離区分のパスへ積まないため、シャドウパスに残る費用が深度配列の消去と保存だけになる。個体だけを外す指定との差が地形の描画費用になる。
    pub(crate) 影キャスター全体有効: bool,
    /// `--lod-probe-step <メートル>`指定でSome。1フレームおきにカメラをZ方向へこの距離だけ動かして戻し、段の境界をまたぐ往復を作る。
    pub(crate) 個体詳細段探査刻み: Option<f32>,
    /// `--dev-ui`指定でtrue。既定はfalse(開発用UIの起動時有効化、判断34。実行中はF3でも切替可能)。
    pub(crate) 開発ui初期有効: bool,
    /// シーンの画素段が本番の色の代わりに出す診断。`--debug-cascade-bands`が距離区分の可視化を、`--debug-shadow-loss`が影可視度と受光距離帯の計器を選ぶ。既定は出さない。
    pub(crate) 画素診断: blitz_render::cascade::画素診断,
    /// `--dump-frame <ベース名>`と`--dump-hdr-frame <ベース名>`指定で、最終フレーム(--frames必須)の読み戻しを書き出す。書き出す画像と外部形式と、同時指定を拒むことは指定の型が持つ。既定は指定なし。
    pub(crate) フレームダンプ先: フレームダンプ指定,
    pub(crate) 読み戻し検収: 読み戻し検収起動設定,
    /// 空と時刻の起動指定。空を描くかどうかは既定では世界の方針が決め、この指定はその上書きである。
    pub(crate) 時間帯: 時間帯起動設定,
    /// 多段シャドウの費用を測るための起動指定。指定が無ければ本番の多段設定と同じ値になる。
    pub(crate) シャドウ計測: シャドウ計測起動設定,
    /// `--no-post`指定でfalse。既定はtrue(HDR中間バッファ+明るさの圧縮パス、判断38・39)。falseならシーンが直接スワップチェーンへ描く構成に戻る(DoD「チェーンの追加・削除可能」の機械実証)。
    pub(crate) ポスト処理有効: bool,
    /// `--exposure <倍率>`指定で変更。既定は1.0(明るさの圧縮前にHDR輝度へ掛ける露出倍率、判断39)。
    pub(crate) 露出: f32,
    /// `--blend <0..1>`指定で変更。既定は0.0(アニメーションクリップ2本のブレンド係数、判断45)。
    pub(crate) ブレンド: f32,
    /// 布シミュレーションの方式(判断52・56)。`--cloth`=吊るし布(全シーン可)、`--cloth-cape`=マント(fox限定、キャラ追従)。既定はなし。
    pub(crate) 布モード: 布モード,
    /// `--report-display-timing`指定でtrue。提示IDと提示待機で実表示間隔を測る。
    ///
    /// 注意: この計測は`vkWaitForPresentKHR`で表示まで描画ループを止める(2026-07-25の実測で毎フレーム約16ms)。フレームペーシングを変えうるため、既存の性能時系列と比較する値を採るときは指定しない。
    /// この指定の有無で条件が変わるので、両条件を比べるときは交互に実行して機材側の時間変動を打ち消すこと。
    pub(crate) 実表示時間報告: bool,
    /// 起動指定が直に選ぶ検証計画。`--window-rebuild`と`--shader-reload`がそれぞれの枝を選び、どちらもピクセル判定を持たない(合否は検収側のxtaskが決める)。
    pub(crate) 検証計画: super::検証計画指定,
    /// チャンクストリーミングの有効化と容量上限。既定は無効(既存のスモークとベンチの挙動を変えない)。
    pub(crate) ストリーミング: ストリーミング起動設定,
    /// 深度プリパスを積むかどうかと、色パスの深度の比べ方。`--depth-prepass`が据える。未指定を`使わない`と同じ値にしないのは、局所可視性補正を宣言した世界が未指定なら方式を引き上げ、明示の`none`とは衝突として落とすためである(2つを1つの値へ潰すと、引き上げが利用者の明示を黙って覆す)。
    pub(crate) 深度プリパス方式: Option<blitz_render::深度プリパス方式>,
    pub(crate) 局所可視性: super::local_visibility_settings::局所可視性の起動指定,
    /// 時間再構成を効かせるかどうかの上書き。既定は世界の宣言に従う。`--no-taa`が使わない側へ落とす。
    pub(crate) 時間再構成: super::temporal_reconstruction_settings::時間再構成方式の起動上書き,
    /// 世界全体とカメラへ加える平行移動。大域ずらし量はカメラ大域原点・照明の大域位置と、起動時シーンとチャンク束の描画の基準原点へ効く。
    /// 基準原点は所有チャンクから導出した後にこの平行移動を合成する。チャンク格子と必要集合の判定は平行移動を含まない世界座標で行うため、位置源には加えない。
    pub(crate) 平行移動: 平行移動起動設定,
    /// `--game`で選ぶ遊ぶゲーム。指定が無ければ「ゲームを遊ばない」であり、ゲーム更新が1つも走らない。
    pub(crate) 遊ぶゲーム: 遊ぶゲームの指定,
}
