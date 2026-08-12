//! 照明問い合わせのセットの宣言の正本と、それを取り込む側の台帳。契約の枝ごとに正本が分かれる。
//! 台帳を検査の手順と分けるのは、契約が増えたときに触るのがこの並びだけになるようにするためである。

/// 正本のパスと、そのファイルが宣言する束縛番号。
pub(super) struct 宣言の正本 {
    pub(super) パス: &'static str,
    pub(super) 束縛番号一覧: &'static [u32],
}

/// 材質のシェーダーが読んでよいのは多段影・ヘッダ・方向光レコード列の3つであり、局所光レコード列とクラスタの2本と点光源の影の立方体配列は局所光の問い合わせの内側だけが読む。
/// 遠方環境は拡散照度・鏡面畳込み・反射率積分表、局所可視度は画素ごとの可視度の画像である。
///
/// 正本を4つに分けるのは、どれも「その番号を宣言してよい唯一のモジュール」が別だからである。
/// とりわけ局所光とクラスタと点光源の影の4つを`local_light_binding.slang`へ分けているのは、材質のシェーダーがその名前を見られないことが先行裁定の実効だからであり、この表がその分割に追随していないと検査が素通りする。
pub(super) const 正本一覧: [宣言の正本; 4] = [
    宣言の正本 {
        パス: "shaders/lighting_query.slang",
        束縛番号一覧: &[0, 1, 2],
    },
    宣言の正本 {
        パス: "shaders/local_light_binding.slang",
        束縛番号一覧: &[3, 8, 9, 10],
    },
    宣言の正本 {
        パス: "shaders/indirect_distant_environment.slang",
        束縛番号一覧: &[4, 5, 6],
    },
    宣言の正本 {
        パス: "shaders/local_visibility_apply.slang",
        束縛番号一覧: &[7],
    },
];

pub(super) const 取り込む側一覧: [&str; 4] = [
    "shaders/scene.slang",
    "shaders/scene_distant_environment.slang",
    "shaders/cloth_draw.slang",
    "shaders/cloth_draw_distant_environment.slang",
];
