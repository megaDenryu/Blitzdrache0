import type { テーマ配色 } from '../../../submodules/VscodeShellLayout/src/テーマ/テーマ型.ts'

export type テーマ名 = '工房' | '夜間'
export const 既定テーマ名: テーマ名 = '工房'

export interface エディター配色 {
    // 背景系
    readonly アプリ背景: string
    readonly ビューポート背景: string
    readonly サイドバー背景: string
    readonly パネル背景: string
    readonly カード背景: string
    readonly カード枠線: string
    readonly カード不透明背景: string
    // 境界線系
    readonly 境界線: string
    readonly 境界線薄: string
    // テキスト系
    readonly テキスト主: string
    readonly テキスト副: string
    readonly テキスト薄: string
    readonly テキストコード: string
    // アイコン系
    readonly アイコン色: string
    readonly フォルダアイコン色: string
    // リスト項目系
    readonly 項目ホバー背景: string
    readonly 項目ホバー文字: string
    readonly 選択背景: string
    readonly 選択枠線: string
    readonly 選択文字: string
    // ボタン系
    readonly ボタン背景: string
    readonly ボタン枠線: string
    readonly ボタン文字: string
    readonly ボタンホバー背景: string
    // アクセント / バッジ
    readonly アクセント背景: string
    readonly アクセントホバー: string
    readonly アクセント文字白: string
    readonly アクセント文字: string
    readonly バッジ背景: string
    readonly バッジ枠線: string
    readonly バッジ文字: string
    // プライマリボタン
    readonly プライマリボタン背景: string
    readonly プライマリボタン枠線: string
    readonly プライマリボタン文字: string
    readonly プライマリボタンホバー: string
    // 危険 / 削除ボタン
    readonly 危険ボタン背景: string
    readonly 危険ボタン枠線: string
    readonly 危険ボタン文字: string
    readonly 危険ボタンホバー: string
    // 土工ボタン (造成・道路の切土盛土等)
    readonly 土工ボタン背景: string
    readonly 土工ボタン枠線: string
    readonly 土工ボタン文字: string
    readonly 土工ボタンホバー: string
    // 非活性状態
    readonly 非活性背景: string
    readonly 非活性文字: string
    // 中立バッジ(情報表示。成功・活性の意味を持たないバッジ)
    readonly 中立バッジ背景: string
    readonly 中立バッジ枠線: string
    readonly 中立バッジ文字: string
    // カードの立体感(白基調3段の面と区別するための影)
    readonly カード影: string
    // ヒント帯(操作案内。カードや情報バッジと区別する灯りの色)
    readonly ヒント背景: string
    // 演出
    readonly ガラス背景ぼかし: string
}

export interface エディターテーマ定義 {
    readonly 名前: テーマ名
    readonly 表示名: string
    readonly 説明: string
    readonly vsl配色: Partial<テーマ配色>
    readonly エディター配色: エディター配色
    readonly 三次元背景色: number
    readonly 大域三次元背景色: number
    // 3Dビューの地形・植生・道路の材質色(テーマごとの見え方を統一するための色トークン)
    readonly 地形基本色: number
    readonly 植生色: number
    readonly 道路色: number
    // 大域編集の地形に入れる標高グラデーション(表示専用。低い側から高い側への3段)
    readonly 標高低色: number
    readonly 標高中色: number
    readonly 標高高色: number
    // 3Dビューの道路点マーカーの色。通常・いま描き足している道の点・選択中・つかんでいる、の4状態を描き分ける
    readonly 道路点通常色: number
    readonly 道路点アクティブ道色: number
    readonly 道路点選択色: number
    readonly 道路点つかんでいる色: number
}
