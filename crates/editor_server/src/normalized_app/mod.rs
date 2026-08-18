//! 経路のパーセント符号を解いてから内側のルーターへ渡す、編集サーバー全体のHTTP境界。
//!
//! `axum::Router::layer`は経路合わせの後で個々の一致した経路（`Route`）へ被さる仕組みであり、
//! 経路合わせそのものより前へ処理を挟めない。そのため経路合わせより外側にこの型自身を
//! `tower::Service`として置き、要求のURIを書き換えてから内側のルーターを呼ぶ
//! （呼び出し連鎖の中の独立した工程。参照: CLAUDE.md「切り出しの根拠義務」第5項。
//! 触れる状態: 内側のルーターの複製を保持するだけであり、他のフィールドは持たない）。
//!
//! `path_normalize`・`percent_decode`はこのモジュールの子として置き`pub(super)`で見せる。
//! 符号を解く工程を経ずに要求のURIを書き換えるコードを他モジュールから足しても、
//! コンパイルが通らない。

mod path_normalize;
mod percent_decode;
mod percent_decode_response;

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{Router, extract::Request, response::Response};
use tower::Service;

use path_normalize::符号を解いた経路を組み立てる;

/// 経路正規化アプリとは、要求の経路のパーセント符号を解いてから内側のルーターへ渡す、
/// 編集サーバーが`axum::serve`へ渡す最上位のHTTPサービスのことである。
#[derive(Clone)]
pub struct 経路正規化アプリ {
    内側のルーター: Router,
}

impl 経路正規化アプリ {
    pub fn 組み立てる(内側のルーター: Router) -> Self {
        Self { 内側のルーター }
    }
}

impl Service<Request> for 経路正規化アプリ {
    type Response = Response;
    type Error = std::convert::Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Response, std::convert::Infallible>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <Router as Service<Request>>::poll_ready(&mut self.内側のルーター, cx)
    }

    fn call(&mut self, mut 要求: Request) -> Self::Future {
        let mut 内側のルーター = self.内側のルーター.clone();
        Box::pin(async move {
            match 符号を解いた経路を組み立てる(要求.uri()) {
                Ok(新しい経路) => {
                    *要求.uri_mut() = 新しい経路;
                    内側のルーター.call(要求).await
                }
                Err(拒否応答) => Ok(*拒否応答),
            }
        })
    }
}

/// `axum::serve`は接続ごとに`Service<IncomingStream>`を1回呼び、その戻り値を要求ごとの
/// `Service<Request>`として使う。`axum::Router`自身がこの橋渡しを実装しているのに合わせ、
/// 同じ形（自身を複製して返すだけ）をこの型にも実装する。
impl<L> Service<axum::serve::IncomingStream<'_, L>> for 経路正規化アプリ
where
    L: axum::serve::Listener,
{
    type Response = Self;
    type Error = std::convert::Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _接続: axum::serve::IncomingStream<'_, L>) -> Self::Future {
        std::future::ready(Ok(self.clone()))
    }
}
