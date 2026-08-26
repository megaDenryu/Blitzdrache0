import type { コード進行, 曲の節 } from '../../../../生成/編集資源契約.ts'

export type 独自の進行を保存する差し戻し = {
    readonly 種類: '独自の進行を保存する'
    readonly 進行名: string
    readonly 変更前進行: コード進行 | null
    readonly 元の位置: number | null
}

export type 独自の進行を削除する差し戻し = {
    readonly 種類: '独自の進行を削除する'
    readonly 削除された進行: コード進行
    readonly 元の位置: number
}

export type 曲の節を追加する差し戻し = {
    readonly 種類: '曲の節を追加する'
    readonly 節の位置: number
}

export type 曲の節を変える差し戻し = {
    readonly 種類: '曲の節を変える'
    readonly 節の位置: number
    readonly 変更前節: 曲の節
}

export type 曲の節を削除する差し戻し = {
    readonly 種類: '曲の節を削除する'
    readonly 節の位置: number
    readonly 削除された節: 曲の節
}

export type 曲の節を並べ替える差し戻し = {
    readonly 種類: '曲の節を並べ替える'
    readonly 元の位置: number
    readonly 先の位置: number
}

export type コード進行と曲構成差し戻し断片 =
    | 独自の進行を保存する差し戻し
    | 独自の進行を削除する差し戻し
    | 曲の節を追加する差し戻し
    | 曲の節を変える差し戻し
    | 曲の節を削除する差し戻し
    | 曲の節を並べ替える差し戻し
