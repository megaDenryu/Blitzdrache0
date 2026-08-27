import { div, span, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { コード進行 } from '../../../../../生成/編集資源契約.ts'
import { 和音の表示名を組み立てる } from '../../../編集モデル/index.ts'
import { 副ボタン, 危険ボタン, 横並び行, 警告の帯 } from '../共通/スタイル.css.ts'
import type { 進行利用状況 } from './進行利用状況計算.ts'
import {
    独自進行の削除影響文を組み立てる,
    独自進行の利用中の要約を組み立てる,
} from './進行利用状況計算.ts'
import { 利用中の札, 独自進行行枠, 独自進行の見出し行, 進行名, 和音要約 } from './スタイル.css.ts'

export interface I独自進行行配線 {
    readonly on編集欄へ読み込む: () => void
    readonly on削除: () => void
}

// 登録済みの独自コード進行1件を見せる行。削除で参照が壊れる先を、行の中に書いて見せる。
export class 独自進行行部品 extends LV2HtmlComponentBase implements I配線可能<I独自進行行配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I独自進行行配線> = new 配線ポート<I独自進行行配線>('独自進行行部品')

    public constructor(進行: コード進行, 利用状況: 進行利用状況) {
        super()
        const 和音の並び = 進行.和音一覧.map((和音) => 和音の表示名を組み立てる(和音)).join(' - ')
        this._componentRoot = div({ class: 独自進行行枠 }).child(
            div({ class: 独自進行の見出し行 }).childs([
                span({ class: 進行名, text: 進行.名前 }),
                span({ class: 和音要約, text: 和音の並び }),
                span({ class: 利用中の札, text: 独自進行の利用中の要約を組み立てる(利用状況) }).setAttribute(
                    'data-使用中',
                    String(利用状況.利用パターン名一覧.length + 利用状況.利用トラック名一覧.length > 0),
                ),
                div({ class: 横並び行 }).childs([
                    button({ class: 副ボタン, text: '編集' })
                        .setTooltip('この進行を編集欄へ読み込む')
                        .onClick(() => this._編集欄へ読み込む要求を伝える()),
                    button({ class: 危険ボタン, text: '削除' })
                        .setTooltip('この独自進行を削除')
                        .onClick(() => this._削除の要求を伝える()),
                ]),
            ]),
        )
        const 影響文 = 独自進行の削除影響文を組み立てる(利用状況)
        if (影響文 !== null) {
            this._componentRoot.child(div({ class: 警告の帯, text: 影響文 }))
        }
    }

    public 配線する(配線: I独自進行行配線): this {
        this._配線.配線する(配線)
        return this
    }

    private _編集欄へ読み込む要求を伝える(): void {
        if (this._配線.配線済みか) this._配線.先.on編集欄へ読み込む()
    }

    private _削除の要求を伝える(): void {
        if (this._配線.配線済みか) this._配線.先.on削除()
    }
}
