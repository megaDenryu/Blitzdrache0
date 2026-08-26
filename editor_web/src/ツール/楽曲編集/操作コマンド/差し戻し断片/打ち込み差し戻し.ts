import type { 打ち込みの対象 } from '../../../../生成/編集資源契約.ts'
import type { トラックの格子 } from '../../../../生成/編集資源契約.ts'

export type 打点を置く差し戻し = {
    readonly 種類: '打点を置く'
    readonly 対象: 打ち込みの対象
    readonly ステップ: number
    readonly 変更前値: number
}

export type 打点を消す差し戻し = {
    readonly 種類: '打点を消す'
    readonly 対象: 打ち込みの対象
    readonly ステップ: number
    readonly 変更前値: number
}

export type 音を伸ばす差し戻し = {
    readonly 種類: '音を伸ばす'
    readonly 対象: 打ち込みの対象
    readonly 始まりのステップ: number
    readonly 変更前値一覧: readonly number[]
}

export type 範囲の打点を消す差し戻し = {
    readonly 種類: '範囲の打点を消す'
    readonly 対象: 打ち込みの対象
    readonly 始まりのステップ: number
    readonly 変更前値一覧: readonly number[]
}

export type パターンの打点を全部消す差し戻し = {
    readonly 種類: 'パターンの打点を全部消す'
    readonly パターンの名乗り: string
    readonly 変更前格子: readonly トラックの格子[]
}

export type 打ち込み差し戻し断片 =
    | 打点を置く差し戻し
    | 打点を消す差し戻し
    | 音を伸ばす差し戻し
    | 範囲の打点を消す差し戻し
    | パターンの打点を全部消す差し戻し
