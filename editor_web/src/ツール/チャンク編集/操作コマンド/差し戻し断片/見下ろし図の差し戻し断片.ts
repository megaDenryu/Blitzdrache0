import type { チャンク座標, 等高線, 大升の塗り } from '../../../../生成/編集資源契約.ts'
import type { 下書きと正本の揃い } from '../../編集モデル/index.ts'

// 見下ろし図の下書きの編集と、下書きからの生成・正本からの逆導出に対する操作の、変更前情報。
// 生成は変更前の格子を丸ごと、下書きの編集は変更前の下書きの該当部分を持つ。どの枝も下書きと正本の揃いを
// 書き換えるため、変更前の揃いを持って戻す。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断2」の表「差し戻しに持つもの」。

export type 等高線を追加する差し戻し = {
    readonly 種類: '等高線を追加する'
    readonly チャンク座標: チャンク座標
    readonly 追加した添字: number
    readonly 変更前の揃い: 下書きと正本の揃い
}
export type 等高線を変更する差し戻し = {
    readonly 種類: '等高線を変更する'
    readonly チャンク座標: チャンク座標
    readonly 添字: number
    readonly 変更前の等高線: 等高線
    readonly 変更前の揃い: 下書きと正本の揃い
}
export type 等高線を削除する差し戻し = {
    readonly 種類: '等高線を削除する'
    readonly チャンク座標: チャンク座標
    readonly 添字: number
    readonly 削除した等高線: 等高線
    readonly 変更前の揃い: 下書きと正本の揃い
}
export type 大升を塗る差し戻し = {
    readonly 種類: '大升を塗る'
    readonly チャンク座標: チャンク座標
    readonly 変更前の大升の一辺の升目数: number
    readonly 変更前の大升の塗り一覧: Array<大升の塗り>
    readonly 変更前の揃い: 下書きと正本の揃い
}
export type 等高線から高さ場を生成する差し戻し = {
    readonly 種類: '等高線から高さ場を生成する'
    readonly チャンク座標: チャンク座標
    readonly 変更前格子データ: Float32Array
    readonly 変更前の揃い: 下書きと正本の揃い
}
export type 大升から地形を生成する差し戻し = {
    readonly 種類: '大升から地形を生成する'
    readonly チャンク座標: チャンク座標
    readonly 変更前格子データ: Float32Array
    readonly 変更前材質データ: Uint8Array
    readonly 変更前の揃い: 下書きと正本の揃い
}
export type 高さ場から等高線を導く差し戻し = {
    readonly 種類: '高さ場から等高線を導く'
    readonly チャンク座標: チャンク座標
    readonly 変更前の等高線一覧: Array<等高線>
    readonly 変更前の揃い: 下書きと正本の揃い
}
export type 高さ場から大升を導く差し戻し = {
    readonly 種類: '高さ場から大升を導く'
    readonly チャンク座標: チャンク座標
    readonly 変更前の大升の一辺の升目数: number
    readonly 変更前の大升の塗り一覧: Array<大升の塗り>
    readonly 変更前の揃い: 下書きと正本の揃い
}

export type 見下ろし図の差し戻し断片 =
    | 等高線を追加する差し戻し
    | 等高線を変更する差し戻し
    | 等高線を削除する差し戻し
    | 大升を塗る差し戻し
    | 等高線から高さ場を生成する差し戻し
    | 大升から地形を生成する差し戻し
    | 高さ場から等高線を導く差し戻し
    | 高さ場から大升を導く差し戻し
