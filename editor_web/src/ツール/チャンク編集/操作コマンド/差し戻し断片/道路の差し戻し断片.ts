import type { 位置3次元, 道路対象 } from '../../../../生成/編集資源契約.ts'
import type { 道路スプライン } from '../../編集モデル/index.ts'

// 道路一覧と道路点に対する操作の、変更前情報。どの道路の何番目かを道路対象が指す。

export type 道路を追加する差し戻し = {
    readonly 種類: '道路を追加する'
    readonly 対象: 道路対象
}
export type 道路を削除する差し戻し = {
    readonly 種類: '道路を削除する'
    readonly 対象: 道路対象
    // 消えた道路そのものを持つ。制御点だけでなく全幅・散布除外バッファ・細分割数まで戻すためである。
    readonly 削除された道路: 道路スプライン
}
export type 道路点を追加する差し戻し = {
    readonly 種類: '道路点を追加する'
    readonly 対象: 道路対象
    readonly 追加された添字: number
}
export type 道路点を挿入する差し戻し = {
    readonly 種類: '道路点を挿入する'
    readonly 対象: 道路対象
    readonly 挿入された添字: number
}
export type 道路点を移動する差し戻し = {
    readonly 種類: '道路点を移動する'
    readonly 対象: 道路対象
    readonly 添字: number
    readonly 変更前位置: 位置3次元
}
export type 道路点を削除する差し戻し = {
    readonly 種類: '道路点を削除する'
    readonly 対象: 道路対象
    readonly 添字: number
    readonly 削除された位置: 位置3次元
}

export type 道路の差し戻し断片 =
    | 道路を追加する差し戻し
    | 道路を削除する差し戻し
    | 道路点を追加する差し戻し
    | 道路点を挿入する差し戻し
    | 道路点を移動する差し戻し
    | 道路点を削除する差し戻し
