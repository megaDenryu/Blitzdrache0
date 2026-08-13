//! 深度領域の組と、その各要素を実装する接点の台帳。

mod camera;
mod camera_compare;
mod shadow;

pub(super) struct 深度契約 {
    pub(super) 領域名: &'static str,
    pub(super) 消去値: f32,
    pub(super) 近面ndc: f32,
    pub(super) 遠面ndc: f32,
    pub(super) 書込比較: &'static str,
    pub(super) 標本比較: &'static str,
}

pub(super) struct 接点 {
    pub(super) 契約: &'static 深度契約,
    pub(super) 項目: &'static str,
    pub(super) パス: &'static str,
    pub(super) 期待する綴り: &'static str,
}

pub(super) const カメラ深度: 深度契約 = 深度契約 {
    領域名: "カメラ逆向き深度",
    消去値: 0.0,
    近面ndc: 1.0,
    遠面ndc: 0.0,
    書込比較: "GREATER / GREATER_OR_EQUAL / EQUAL",
    標本比較: "比較標本器なし",
};

pub(super) const 光源影深度: 深度契約 = 深度契約 {
    領域名: "光源影標準深度",
    消去値: 1.0,
    近面ndc: 0.0,
    遠面ndc: 1.0,
    書込比較: "LESS",
    標本比較: "LESS_OR_EQUAL",
};

pub(super) fn 全接点() -> impl Iterator<Item = &'static 接点> {
    camera::接点一覧
        .iter()
        .chain(camera_compare::接点一覧.iter())
        .chain(shadow::接点一覧.iter())
}
