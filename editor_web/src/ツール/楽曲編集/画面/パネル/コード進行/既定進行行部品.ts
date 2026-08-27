import { div, span, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 既定のコード進行 } from '../../../../../生成/編集資源契約.ts'
import { 和音の表示名を組み立てる } from '../../../編集モデル/index.ts'
import { 副ボタン } from '../共通/スタイル.css.ts'
import { 既定進行行枠, 進行名, 和音要約 } from './スタイル.css.ts'

export interface I既定進行行配線 {
    readonly onひな形として読み込む: () => void
}

// 既定のコード進行1件を見せる行。既定の進行は書き換えられないため、
// できる操作は編集欄へひな形として写すことだけである。
export class 既定進行行部品 extends LV2HtmlComponentBase implements I配線可能<I既定進行行配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I既定進行行配線> = new 配線ポート<I既定進行行配線>('既定進行行部品')

    public constructor(進行: 既定のコード進行) {
        super()
        const 和音の並び = 進行.和音一覧.map((和音) => 和音の表示名を組み立てる(和音)).join(' - ')
        this._componentRoot = div({ class: 既定進行行枠 }).childs([
            span({ class: 進行名, text: 進行.表示名 }),
            span({ class: 和音要約, text: 和音の並び }),
            button({ class: 副ボタン, text: 'ひな形として読込' })
                .setTooltip('この既定進行を編集欄へ写す')
                .onClick(() => this._ひな形として読み込む要求を伝える()),
        ])
    }

    public 配線する(配線: I既定進行行配線): this {
        this._配線.配線する(配線)
        return this
    }

    private _ひな形として読み込む要求を伝える(): void {
        if (this._配線.配線済みか) this._配線.先.onひな形として読み込む()
    }
}
