import type { 位置3次元, チャンク座標, 建物の配置, 散布の設定 } from '../../../../生成/編集資源契約.ts'

// 高さ場・地表材質・建物・散布に対する操作の、変更前情報。格子は丸ごとの複製で持つ。

export type 造成筆致差し戻し = {
    readonly 種類: '造成筆致'
    readonly 対象チャンク: チャンク座標 | null
    readonly 変更前格子データ: Float32Array
}
export type 材質の筆致差し戻し = {
    readonly 種類: '材質の筆致'
    readonly 対象チャンク: チャンク座標
    readonly 変更前材質データ: Uint8Array
}
export type 建物を配置する差し戻し = {
    readonly 種類: '建物を配置する'
    readonly チャンク座標: チャンク座標
    readonly 建物識別子: string
}
export type 建物を移動する差し戻し = {
    readonly 種類: '建物を移動する'
    readonly チャンク座標: チャンク座標
    readonly 識別子: string
    readonly 変更前位置: 位置3次元
    readonly 変更前向きラジアン: number
}
export type 建物を削除する差し戻し = {
    readonly 種類: '建物を削除する'
    readonly チャンク座標: チャンク座標
    readonly 削除された建物: 建物の配置
}
export type 散布設定を変更する差し戻し = {
    readonly 種類: '散布設定を変更する'
    readonly チャンク座標: チャンク座標
    readonly 変更前設定: 散布の設定
}
export type 道路に合わせて切土盛土する差し戻し = {
    readonly 種類: '道路に合わせて切土盛土する'
    readonly チャンク座標: チャンク座標
    readonly 変更前格子データ: Float32Array
}
export type 建物基礎を平坦化する差し戻し = {
    readonly 種類: '建物基礎を平坦化する'
    readonly チャンク座標: チャンク座標
    readonly 変更前格子データ: Float32Array
}
export type 急勾配を岩肌へベイクする差し戻し = {
    readonly 種類: '急勾配を岩肌へベイクする'
    readonly チャンク座標: チャンク座標
    readonly 変更前材質データ: Uint8Array
}
export type 道路下を泥へベイクする差し戻し = {
    readonly 種類: '道路下を泥へベイクする'
    readonly チャンク座標: チャンク座標
    readonly 変更前材質データ: Uint8Array
}

export type 地形と建物の差し戻し断片 =
    | 造成筆致差し戻し
    | 材質の筆致差し戻し
    | 建物を配置する差し戻し
    | 建物を移動する差し戻し
    | 建物を削除する差し戻し
    | 散布設定を変更する差し戻し
    | 道路に合わせて切土盛土する差し戻し
    | 建物基礎を平坦化する差し戻し
    | 急勾配を岩肌へベイクする差し戻し
    | 道路下を泥へベイクする差し戻し
