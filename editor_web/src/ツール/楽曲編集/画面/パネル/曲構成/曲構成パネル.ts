import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 曲の節の繰り返し回数の下限, type 楽曲 } from '../../../../../生成/編集資源契約.ts'
import { パネル外枠, パネル見出し } from '../共通/スタイル.css.ts'
import { 曲構成行部品 } from './曲構成行部品.ts'
import { 曲構成の概要表示 } from './曲構成の概要表示.ts'
import { 節追加ボタン } from './節追加ボタン.ts'
import { 節一覧コンテナ } from './スタイル.css.ts'

export interface I曲構成パネル配線 {
    readonly on節追加: (パターンの名乗り: string, 繰り返し回数: number) => void
    readonly on節変更: (節の位置: number, 新しいパターンの名乗り: string, 新しい繰り返し回数: number) => void
    readonly on節削除: (節の位置: number) => void
    readonly on節並べ替え: (元の位置: number, 先の位置: number) => void
}

// 楽曲の曲構成（パターンの並び順・繰り返し回数）と曲全体の長さを編集・表示するパネル。
export class 曲構成パネル extends LV2HtmlComponentBase implements I配線可能<I曲構成パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I曲構成パネル配線> = new 配線ポート<I曲構成パネル配線>('曲構成パネル')
    private readonly _概要表示: 曲構成の概要表示 = new 曲構成の概要表示()
    private readonly _節一覧コンテナ: DivC = div({ class: 節一覧コンテナ })
    private readonly _追加ボタン: 節追加ボタン = new 節追加ボタン()
    private _行一覧: 曲構成行部品[] = []
    private _選択中パターンの名乗り: string | null = null

    public constructor(初期楽曲: 楽曲, 初期選択パターン名乗り: string | null) {
        super()
        this._componentRoot = div({ class: パネル外枠 }).childs([
            div({ class: パネル見出し, text: '曲構成 (アレンジ)' }),
            this._概要表示,
            this._節一覧コンテナ,
            this._追加ボタン,
        ])
        this.表示を更新する(初期楽曲, 初期選択パターン名乗り)
    }

    public 配線する(配線: I曲構成パネル配線): this {
        this._配線.配線する(配線)
        this._追加ボタン.onClick(() => this._節の追加を伝える())
        this._行を配線する()
        return this
    }

    public 表示を更新する(楽曲: 楽曲, 選択中パターン名乗り: string | null): void {
        this._選択中パターンの名乗り = 選択中パターン名乗り
        this._概要表示.曲構成を反映する(楽曲, 選択中パターン名乗り)
        this._追加ボタン.選択中パターンを反映する(選択中パターン名乗り)
        this._行一覧を再構築する(楽曲)
        this._行を配線する()
    }

    public override delete(): void {
        for (const 行 of this._行一覧) 行.delete()
        this._行一覧 = []
        this._概要表示.delete()
        this._追加ボタン.delete()
        super.delete()
    }

    private _節の追加を伝える(): void {
        if (this._配線.配線済みか && this._選択中パターンの名乗り !== null) {
            this._配線.先.on節追加(this._選択中パターンの名乗り, 曲の節の繰り返し回数の下限)
        }
    }

    private _行一覧を再構築する(楽曲: 楽曲): void {
        for (const 行 of this._行一覧) 行.delete()
        this._節一覧コンテナ.clearChildren()
        const 全節数 = 楽曲.曲構成.length
        this._行一覧 = 楽曲.曲構成.map((節, 位置) => {
            const 行 = new 曲構成行部品(節, 位置, 全節数, 楽曲.パターン一覧)
            this._節一覧コンテナ.child(行)
            return 行
        })
    }

    private _行を配線する(): void {
        if (!this._配線.配線済みか) return
        for (const [位置, 行] of this._行一覧.entries()) {
            行.配線する({
                on節変更: (新名乗り, 新回数) => this._配線.先.on節変更(位置, 新名乗り, 新回数),
                on上へ移動: () => this._配線.先.on節並べ替え(位置, 位置 - 1),
                on下へ移動: () => this._配線.先.on節並べ替え(位置, 位置 + 1),
                on削除: () => this._配線.先.on節削除(位置),
            })
        }
    }
}
