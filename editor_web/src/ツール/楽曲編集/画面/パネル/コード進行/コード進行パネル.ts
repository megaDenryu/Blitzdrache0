import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 既定のコード進行一覧, type 楽曲, type コード進行 } from '../../../../../生成/編集資源契約.ts'
import { パネル外枠, パネル見出し, 区画, 区画見出し } from '../共通/スタイル.css.ts'
import { 独自進行の利用状況を調べる } from './進行利用状況計算.ts'
import { 既定進行行部品 } from './既定進行行部品.ts'
import { 独自進行行部品 } from './独自進行行部品.ts'
import { 独自進行編集欄 } from './独自進行編集欄.ts'
import { 一覧枠, 和音要約 } from './スタイル.css.ts'

export interface Iコード進行パネル配線 {
    readonly on独自進行保存: (進行: コード進行) => void
    readonly on独自進行削除: (名前: string) => void
}

// 既定のコード進行の閲覧および独自コード進行の作成・編集・削除を行うパネル。
export class コード進行パネル extends LV2HtmlComponentBase implements I配線可能<Iコード進行パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iコード進行パネル配線> = new 配線ポート<Iコード進行パネル配線>('コード進行パネル')
    private readonly _既定進行一覧枠: DivC = div({ class: 一覧枠 })
    private readonly _独自進行一覧枠: DivC = div({ class: 一覧枠 })
    private readonly _編集欄: 独自進行編集欄 = new 独自進行編集欄()
    private _既定進行行一覧: 既定進行行部品[] = []
    private _独自進行行一覧: 独自進行行部品[] = []

    public constructor(初期楽曲: 楽曲) {
        super()
        this._既定進行一覧を組み立てる()
        this._componentRoot = this._ルートを構築する()
        this.表示を更新する(初期楽曲)
    }

    public 配線する(配線: Iコード進行パネル配線): this {
        this._配線.配線する(配線)
        this._編集欄.配線する({
            on保存: (進行) => {
                if (this._配線.配線済みか) this._配線.先.on独自進行保存(進行)
            },
        })
        return this
    }

    public 表示を更新する(楽曲: 楽曲): void {
        for (const 行 of this._独自進行行一覧) 行.delete()
        this._独自進行一覧枠.clearChildren()
        if (楽曲.独自進行一覧.length === 0) {
            this._独自進行行一覧 = []
            this._独自進行一覧枠.child(span({ class: 和音要約, text: '登録されている独自の進行はありません。' }))
            return
        }
        this._独自進行行一覧 = 楽曲.独自進行一覧.map((進行) => {
            const 利用状況 = 独自進行の利用状況を調べる(進行.名前, 楽曲.パターン一覧, 楽曲.トラック構成)
            const 行 = new 独自進行行部品(進行, 利用状況).配線する({
                on編集欄へ読み込む: () => this._編集欄.進行を読み込む(進行.名前, 進行.和音一覧),
                on削除: () => this._独自進行の削除を伝える(進行.名前),
            })
            this._独自進行一覧枠.child(行)
            return 行
        })
    }

    public override delete(): void {
        for (const 行 of this._既定進行行一覧) 行.delete()
        for (const 行 of this._独自進行行一覧) 行.delete()
        this._既定進行行一覧 = []
        this._独自進行行一覧 = []
        this._編集欄.delete()
        super.delete()
    }

    private _独自進行の削除を伝える(名前: string): void {
        if (this._配線.配線済みか) this._配線.先.on独自進行削除(名前)
    }

    private _既定進行一覧を組み立てる(): void {
        this._既定進行行一覧 = 既定のコード進行一覧.map((進行) => {
            const 行 = new 既定進行行部品(進行).配線する({
                onひな形として読み込む: () => this._編集欄.進行を読み込む(`${進行.識別子}の写し`, 進行.和音一覧),
            })
            this._既定進行一覧枠.child(行)
            return 行
        })
    }

    private _ルートを構築する(): DivC {
        return div({ class: パネル外枠 }).childs([
            div({ class: パネル見出し, text: 'コード進行' }),
            div({ class: 区画 }).childs([
                div({ class: 区画見出し, text: '既定のコード進行 (参照専用)' }),
                this._既定進行一覧枠,
            ]),
            div({ class: 区画 }).childs([
                div({ class: 区画見出し, text: '独自のコード進行' }),
                this._独自進行一覧枠,
            ]),
            div({ class: 区画 }).childs([
                div({ class: 区画見出し, text: '独自の進行を作成・編集' }),
                this._編集欄,
            ]),
        ])
    }
}
