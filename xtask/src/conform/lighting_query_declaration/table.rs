//! 照明問い合わせのセットの宣言の正本と、それを取り込む側の台帳。契約の枝ごとに正本が分かれる。
//! 台帳を検査の手順と分けるのは、契約が増えたときに触るのがこの並びだけになるようにするためである。

/// 正本のパスと、そのファイルが宣言するバインディング番号。
pub(super) struct 宣言の正本 {
    pub(super) パス: &'static str,
    pub(super) バインディング番号一覧: &'static [u32],
}

/// 直接光と影は影・ヘッダ・方向光レコード列・局所光レコード列とクラスタ格子の2本、遠方環境は拡散照度・鏡面畳込み・
/// 反射率積分表、局所可視度は画素ごとの可視度の画像である。
/// クラスタの2本と局所可視度の1つは両方の契約が宣言するが、宣言の在り処が消費する側のモジュールであるため正本を分ける。
pub(super) const 正本一覧: [宣言の正本; 3] = [
    宣言の正本 {
        パス: "shaders/lighting_query.slang",
        バインディング番号一覧: &[0, 1, 2, 3, 8, 9],
    },
    宣言の正本 {
        パス: "shaders/indirect_distant_environment.slang",
        バインディング番号一覧: &[4, 5, 6],
    },
    宣言の正本 {
        パス: "shaders/local_visibility_apply.slang",
        バインディング番号一覧: &[7],
    },
];

pub(super) const 取り込む側一覧: [&str; 4] = [
    "shaders/scene.slang",
    "shaders/scene_distant_environment.slang",
    "shaders/cloth_draw.slang",
    "shaders/cloth_draw_distant_environment.slang",
];
