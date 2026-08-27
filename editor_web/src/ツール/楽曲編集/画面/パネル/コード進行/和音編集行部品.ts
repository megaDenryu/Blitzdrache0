import { div, span, select, button, DivC, SelectC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 和音 } from '../../../../../生成/編集資源契約.ts'
import { 選択セレクト, 危険な記号ボタン } from '../共通/スタイル.css.ts'
import {
    根音の選択肢一覧を組み立てる,
    和音の種類の選択肢一覧を組み立てる,
    続くステップ数の選択肢一覧を組み立てる,
    綴りから根音を復元する,
    綴りから和音の種類を復元する,
    綴りから続くステップ数を復元する,
} from './和音の欄の値.ts'
import { 和音行枠, 和音番号, 和音の欄 } from './スタイル.css.ts'

export interface I和音編集行配線 {
    readonly on和音変更: (新しい和音: 和音) => void
    readonly on削除: () => void
}

// 独自進行を構成する和音1件（根音・種類・続くステップ数）を編集する行コンポーネント。
export class 和音編集行部品 extends LV2HtmlComponentBase implements I配線可能<I和音編集行配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I和音編集行配線> = new 配線ポート<I和音編集行配線>('和音編集行部品')
    private readonly _根音選択: SelectC
    private readonly _種類選択: SelectC
    private readonly _ステップ選択: SelectC
    private readonly _削除ボタン: ButtonC

    public constructor(対象の和音: 和音, 位置: number) {
        super()
        const 欄の見た目 = `${選択セレクト} ${和音の欄}`
        this._根音選択 = select({ class: 欄の見た目 }).setOptions(根音の選択肢一覧を組み立てる(対象の和音.根音))
        this._種類選択 = select({ class: 欄の見た目 })
            .setOptions(和音の種類の選択肢一覧を組み立てる(対象の和音.種類))
        this._ステップ選択 = select({ class: 欄の見た目 })
            .setOptions(続くステップ数の選択肢一覧を組み立てる(対象の和音.続くステップ数))
        this._削除ボタン = button({ class: 危険な記号ボタン, text: '×' }).setTooltip('この和音を削除')
        this._componentRoot = this._ルートを構築する(位置)
    }

    public 配線する(配線: I和音編集行配線): this {
        this._配線.配線する(配線)
        this._根音選択.onSelectChange(() => this._書き換えられた和音を伝える())
        this._種類選択.onSelectChange(() => this._書き換えられた和音を伝える())
        this._ステップ選択.onSelectChange(() => this._書き換えられた和音を伝える())
        this._削除ボタン.onClick(() => {
            if (this._配線.配線済みか) this._配線.先.on削除()
        })
        return this
    }

    public override delete(): void {
        this._根音選択.delete()
        this._種類選択.delete()
        this._ステップ選択.delete()
        this._削除ボタン.delete()
        super.delete()
    }

    private _書き換えられた和音を伝える(): void {
        if (!this._配線.配線済みか) return
        this._配線.先.on和音変更({
            根音: 綴りから根音を復元する(this._根音選択.getValue()),
            種類: 綴りから和音の種類を復元する(this._種類選択.getValue()),
            続くステップ数: 綴りから続くステップ数を復元する(this._ステップ選択.getValue()),
        })
    }

    private _ルートを構築する(位置: number): DivC {
        return div({ class: 和音行枠 }).childs([
            span({ class: 和音番号, text: `#${位置 + 1}` }),
            this._根音選択,
            this._種類選択,
            this._ステップ選択,
            this._削除ボタン,
        ])
    }
}
