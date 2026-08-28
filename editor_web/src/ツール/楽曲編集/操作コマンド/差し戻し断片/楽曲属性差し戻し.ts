import type {
    コード進行参照,
    ミキサー設定,
    楽器,
} from '../../../../生成/編集資源契約.ts'

export type テンポを変える差し戻し = {
    readonly 種類: 'テンポを変える'
    readonly 変更前テンポ: number
}

export type ミキサー設定を変える差し戻し = {
    readonly 種類: 'ミキサー設定を変える'
    readonly 変更前ミキサー設定: ミキサー設定
}

export type 楽曲の表示名を変える差し戻し = {
    readonly 種類: '楽曲の表示名を変える'
    readonly 変更前表示名: string
}

export type トラックの楽器を変える差し戻し = {
    readonly 種類: 'トラックの楽器を変える'
    readonly トラックの位置: number
    readonly 変更前楽器: 楽器
}

export type トラックの音量を変える差し戻し = {
    readonly 種類: 'トラックの音量を変える'
    readonly トラックの位置: number
    readonly 変更前音量: number
}

export type トラックの進行の割り当てを変える差し戻し = {
    readonly 種類: 'トラックの進行の割り当てを変える'
    readonly トラックの位置: number
    readonly 変更前進行の割り当て: コード進行参照 | null
}

export type 楽曲属性差し戻し断片 =
    | テンポを変える差し戻し
    | ミキサー設定を変える差し戻し
    | 楽曲の表示名を変える差し戻し
    | トラックの楽器を変える差し戻し
    | トラックの音量を変える差し戻し
    | トラックの進行の割り当てを変える差し戻し
