import type {
    コード進行参照,
    トラックの格子,
    パターン,
    曲の節,
} from '../../../../生成/編集資源契約.ts'

export type 削除された曲の節の記録 = {
    readonly 節の位置: number
    readonly 節: 曲の節
}

export type パターンを追加する差し戻し = {
    readonly 種類: 'パターンを追加する'
    readonly 名乗り: string
}

export type パターンを削除する差し戻し = {
    readonly 種類: 'パターンを削除する'
    readonly 削除されたパターン: パターン
    readonly 元の位置: number
    readonly 連動して削除された節一覧: readonly 削除された曲の節の記録[]
}

export type パターンの進行を変える差し戻し = {
    readonly 種類: 'パターンの進行を変える'
    readonly 名乗り: string
    readonly 変更前進行の参照: コード進行参照
    readonly 変更前格子: readonly トラックの格子[]
}

export type パターンの表示名を変える差し戻し = {
    readonly 種類: 'パターンの表示名を変える'
    readonly 名乗り: string
    readonly 変更前表示名: string
}

export type パターン差し戻し断片 =
    | パターンを追加する差し戻し
    | パターンを削除する差し戻し
    | パターンの進行を変える差し戻し
    | パターンの表示名を変える差し戻し
