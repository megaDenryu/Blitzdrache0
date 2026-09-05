//! 建物の格子の経路の往復と、変換できない宣言が正本を書き換えずに拒まれることを確かめる。
//!
//! 拒否の検査を外部アセットの有無に依らせないのは、升目の宣言の破れが部品の実体を1つも読まずに決まるためである。
//! カタログの組み直しまで通る受理の検査だけが外部アセットを要る。

#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use crate::common::{
    初期の格子のjson, 升目を空にした格子のjson, 外部アセットの置き場があるか, 試験の建物定義の識別子
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

async fn 要求を送る(ルーター: &editor_server::経路正規化アプリ, 要求: Request<Body>) -> (StatusCode, String) {
    let 応答 = ルーター.clone().oneshot(要求).await.unwrap();
    let 状態 = 応答.status();
    let 本体 = axum::body::to_bytes(応答.into_body(), usize::MAX).await.unwrap();
    (状態, String::from_utf8_lossy(&本体).into_owned())
}

fn 格子を保存する要求(識別子: &str, 本文: String) -> Request<Body> {
    Request::put(format!("/api/建物/{識別子}/格子"))
        .header("content-type", "application/json")
        .body(Body::from(本文))
        .unwrap()
}

fn 格子のパス(一時: &crate::common::一時プロジェクト) -> std::path::PathBuf {
    一時
        .ルート()
        .join("editor_data")
        .join("建物")
        .join(試験の建物定義の識別子)
        .join("格子.json")
}

#[tokio::test]
async fn 升目を1つも持たない格子は拒まれ正本が作られない() {
    let 一時 = crate::common::一時プロジェクト::生成する("building_grid_empty");
    let ルーター = crate::common::ルーターを作る(&一時);
    let 要求 = 格子を保存する要求(試験の建物定義の識別子, 升目を空にした格子のjson().to_string());
    let (状態, 本文) = 要求を送る(&ルーター, 要求).await;
    assert_eq!(状態, StatusCode::UNPROCESSABLE_ENTITY, "本文: {本文}");
    assert!(!格子のパス(&一時).is_file(), "拒まれた格子の正本が作られている");
}

#[tokio::test]
async fn 経路と本文の建物定義の識別子が食い違う要求は拒まれる() {
    let 一時 = crate::common::一時プロジェクト::生成する("building_grid_id_mismatch");
    let ルーター = crate::common::ルーターを作る(&一時);
    let 要求 = 格子を保存する要求("別の名前", 初期の格子のjson().to_string());
    let (状態, 本文) = 要求を送る(&ルーター, 要求).await;
    assert_eq!(状態, StatusCode::BAD_REQUEST, "本文: {本文}");
}

#[tokio::test]
async fn 未保存の建物の格子はnullで返り一覧は空になる() {
    let 一時 = crate::common::一時プロジェクト::生成する("building_grid_absent");
    let ルーター = crate::common::ルーターを作る(&一時);
    let 経路 = format!("/api/建物/{試験の建物定義の識別子}/格子");
    let (状態, 本文) = 要求を送る(&ルーター, Request::get(経路).body(Body::empty()).unwrap()).await;
    assert_eq!(状態, StatusCode::OK);
    assert_eq!(本文.trim(), "null");
    let (一覧の状態, 一覧の本文) = 要求を送る(&ルーター, Request::get("/api/建物一覧").body(Body::empty()).unwrap()).await;
    assert_eq!(一覧の状態, StatusCode::OK);
    assert_eq!(一覧の本文.trim(), "[]");
}

/// 道具が作る初期の格子が保存の検査を通り、一覧とカタログの両方に現れることを確かめる。
/// 部品の実体を読むため、置き場が無い環境では飛ばす。
#[tokio::test]
async fn 初期の格子は保存されて一覧とカタログに載る() {
    if !外部アセットの置き場があるか() {
        println!("外部アセットの置き場が無いため、建物の格子の受理の検査をスキップする");
        return;
    }
    let 一時 = crate::common::一時プロジェクト::生成する("building_grid_accept");
    let ルーター = crate::common::ルーターを作る(&一時);
    let 要求 = 格子を保存する要求(試験の建物定義の識別子, 初期の格子のjson().to_string());
    let (状態, 本文) = 要求を送る(&ルーター, 要求).await;
    assert_eq!(状態, StatusCode::NO_CONTENT, "本文: {本文}");
    assert!(格子のパス(&一時).is_file(), "受理された格子の正本が書かれていない");

    let (_, 一覧) = 要求を送る(&ルーター, Request::get("/api/建物一覧").body(Body::empty()).unwrap()).await;
    assert!(一覧.contains(試験の建物定義の識別子), "一覧に載っていない: {一覧}");
    let (_, カタログ) = 要求を送る(&ルーター, Request::get("/api/建物外形カタログ").body(Body::empty()).unwrap()).await;
    assert!(カタログ.contains(試験の建物定義の識別子), "カタログに載っていない: {カタログ}");
    let 経路 = format!("/api/建物/{試験の建物定義の識別子}/格子");
    let (_, 読み戻し) = 要求を送る(&ルーター, Request::get(経路).body(Body::empty()).unwrap()).await;
    assert!(読み戻し.contains("扉枠付きの壁"), "読み戻した格子が保存した内容と違う: {読み戻し}");
}
