# Blitzdrache0

Blitz（稲妻）+ Drache（竜）。Rust + Vulkan による自作ゲームエンジン。

デバイスの理論値性能を志向し、汎用エンジン（Unity/Unreal）が払う汎用性の税金を避けるため、
Vulkan を生バインディング（ash）で直接扱う。安全性は「あらゆる使い方を安全にする汎用ラッパー」
ではなく「このエンジンの使い方だけを露出する狭い型安全層」で担保する。

## アーキテクチャ

```
ゲームロジック（安全なRust）
   ↓
blitz_engine: シーン・アセット・マテリアル（安全なRust）
   ↓
blitz_render: レンダーグラフ + リソースシステム ← 自作の型安全層 = エンジン中核
   ↓
ash（unsafe は blitz_render の実装内部に封じ込め）
```

層 = クレートとして分割し、依存方向を Cargo でコンパイル時に強制する。
`blitz_engine` / `blitz_app` は `#![forbid(unsafe_code)]` を宣言しており、
unsafe が blitz_render の外に書かれることも機械的に禁止される。

| クレート | 役割 | 制約 |
|---|---|---|
| `crates/blitz_app` | 起動バイナリ 兼 コンポジションルート。ウィンドウ生成と依存の配線のみ | unsafe禁止。ロジックを書かない |
| `crates/blitz_engine` | シーン・アセット・マテリアル。描画内容をレンダーグラフへ翻訳 | unsafe禁止。ash に依存しない |
| `crates/blitz_render` | レンダーグラフ + リソースシステム。Vulkan の unsafe をここに封じ込め | 公開APIに ash の型を露出しない |

## 利用ライブラリ

| ライブラリ | 何か | 採用理由 |
|---|---|---|
| [ash](https://github.com/ash-rs/ash) | Vulkan の生バインディング。C API とほぼ 1:1 | Vulkan 仕様書・既存チュートリアルの知識がそのまま使える最薄の層。薄いのでライブラリ自体の信頼性が問題にならない。wgpu は WebGPU の共通部分集合に縛られ（バインドレス・メッシュシェーダー・マルチキュー等が制約）、vulkano は汎用安全化の実行時追跡コストと保守体制の問題で不採用 |
| [ash-window](https://crates.io/crates/ash-window) | raw-window-handle → Vulkan サーフェス変換の小さな橋渡し | ウィンドウハンドル（Windows なら HWND）から `vkCreateWin32SurfaceKHR` 等の OS 別サーフェス生成を吸収する定型コードを省く |
| [winit](https://github.com/rust-windowing/winit) | ウィンドウ生成とイベントループ。Win32 の RegisterClass / CreateWindowEx / メッセージループ / WndProc 一式を OS 差異ごと抽象化した純 Rust ライブラリ。描画機能は持たない | 生 Win32 自作は全部 unsafe FFI になり封じ込め設計を性能と無関係な場所で破る。コールバック型イベントループ（ApplicationHandler）は Win32 の WndProc と同根の OS 側の現実であり、`ControlFlow::Poll` + `request_redraw` の定石でゲームループを構成する。ループ所有権が必要になったら `pump_app_events` に移行可能。winit に触るのは blitz_app のみで、差し替えてもエンジン本体は不変 |
| [raw-window-handle](https://crates.io/crates/raw-window-handle) | ウィンドウハンドルの共通インターフェース | winit と blitz_render の間の結合をこの標準トレイトだけにし、ウィンドウライブラリを差し替え可能に保つ |
| [glam](https://github.com/bitshifter/glam-rs) | ゲーム向け線形代数（Vec3 / Mat4 / Quat 等）。SIMD 最適化済み | Rust ゲーム開発の事実上の標準。自作する価値のない基盤数学 |
| [thiserror](https://github.com/dtolnay/thiserror) | 型付きエラー定義の derive マクロ | 文字列エラーではなく判別可能なエラー型を層ごとに定義するため |

## ビルド

```
cargo build        # 全クレート
cargo run -p blitz_app
```

開発時は Vulkan validation layer（VK_LAYER_KHRONOS_validation、同期検証含む）を常時有効にする。

## ドキュメント

- [開発計画](_doc/計画/開発計画.md) — ゴール・非ゴール・マイルストーン（M0〜M8）・
  自律ループの運用規約と判断基準。開発の現在地と次のタスクはここから特定する
- 意思決定の経緯: `_doc/開発スレッド/` — 技術選定・却下案・方向転換はここに記録する
- [イベントループとフレームペーシング](_doc/設計/イベントループとフレームペーシング.md) —
  16.6msを誰が作るか（提示モードが主役）、winit所有ループと自前所有ループ（pump_app_events）の
  2パターン、およびA→Bの移行手順
- シェーダー言語は Slang を採用予定（SPIR-V/DXIL/Metal 出力可。API より移植性に効くため先に固定）
- 当面 Windows + Vulkan のみ。Vulkan 自体がポータブルなため Linux/Android/MoltenVK 展開は後から可能。
  RHI（マルチAPI抽象層）はコンソール対応が現実になった時点で切り出す
