import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲, 楽器, コード進行参照 } from '../../../../../生成/編集資源契約.ts'
import { トラック設定行 } from './トラック設定行.ts'
import { パネル外枠, パネル見出し } from '../共通/スタイル.css.ts'
import { トラック行一覧 } from './スタイル.css.ts'

export interface Iトラック設定パネル配線 {
    readonly on楽器変更: (トラックの位置: number, 新しい楽器: 楽器) => void
    readonly on音量変更: (トラックの位置: number, 新しい音量: number) => void
    readonly on進行割り当て変更: (トラックの位置: number, 新しい進行の割り当て: コード進行参照 | null) => void
}

// 楽曲が保持する全トラックの楽器・音量・進行割り当てを一覧編集するパネル。
export class トラック設定パネル extends LV2HtmlComponentBase implements I配線可能<Iトラック設定パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iトラック設定パネル配線> = new 配線ポート<Iトラック設定パネル配線>('トラック設定パネル')
    private readonly _行コンテナ: DivC
    private _行一覧: トラック設定行[] = []

    public constructor(初期楽曲: 楽曲) {
        super()
        this._行コンテナ = div({ class: トラック行一覧 })
        this._componentRoot = div({ class: パネル外枠 }).childs([
            div({ class: パネル見出し, text: 'トラックの設定' }),
            this._行コンテナ,
        ])
        this._行一覧を再構築する(初期楽曲)
    }

    public 配線する(配線: Iトラック設定パネル配線): this {
        this._配線.配線する(配線)
        this._行を配線する()
        return this
    }

    public 表示を更新する(楽曲: 楽曲): void {
        if (this._行一覧.length !== 楽曲.トラック構成.length) {
            this._行一覧を再構築する(楽曲)
            this._行を配線する()
            return
        }

        for (const [位置, トラック] of 楽曲.トラック構成.entries()) {
            const 行 = this._行一覧[位置]
            if (行 === undefined) {
                throw new Error(`トラックの本数は一致しているのに行がありません: 位置=${位置}`)
            }
            行.表示を更新する(トラック, 楽曲.独自進行一覧)
        }
    }

    public override delete(): void {
        for (const 行 of this._行一覧) 行.delete()
        this._行一覧 = []
        super.delete()
    }

    private _行一覧を再構築する(楽曲: 楽曲): void {
        for (const 行 of this._行一覧) 行.delete()
        this._行コンテナ.clearChildren()
        this._行一覧 = 楽曲.トラック構成.map((トラック) => {
            const 行 = new トラック設定行(トラック, 楽曲.独自進行一覧)
            this._行コンテナ.child(行)
            return 行
        })
    }

    private _行を配線する(): void {
        if (!this._配線.配線済みか) return
        for (const [位置, 行] of this._行一覧.entries()) {
            行.配線する({
                on楽器変更: (新楽器) => this._配線.先.on楽器変更(位置, 新楽器),
                on音量変更: (新音量) => this._配線.先.on音量変更(位置, 新音量),
                on進行割り当て変更: (新進行) => this._配線.先.on進行割り当て変更(位置, 新進行),
            })
        }
    }
}
