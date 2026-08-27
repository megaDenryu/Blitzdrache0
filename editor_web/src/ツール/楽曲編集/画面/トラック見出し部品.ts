import { div, span, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { トラック定義 } from '../../../生成/編集資源契約.ts'
import { トラックヘッダー, トラック名, トラック属性群, 属性バッジ } from './スタイル.css.ts'

// 1つのトラックの表示名・種類・楽器・音量・進行割り当てを表示する部品。
export class トラック見出し部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _トラック名要素: DivC
    private readonly _種類バッジ: DivC
    private readonly _楽器バッジ: DivC
    private readonly _音量バッジ: DivC
    private readonly _進行バッジ: DivC

    public constructor(トラック: トラック定義) {
        super()
        this._トラック名要素 = div({ class: トラック名, text: トラック.表示名 })
        this._種類バッジ = div({ class: 属性バッジ, text: トラック.種類 })
        this._楽器バッジ = div({ class: 属性バッジ, text: `楽器: ${トラック.楽器}` })
        this._音量バッジ = div({ class: 属性バッジ, text: `音量: ${Math.round(トラック.音量 * 100)}%` })
        this._進行バッジ = div({ class: 属性バッジ, text: トラック見出し部品.進行割り当て文言(トラック) })

        this._componentRoot = div({ class: トラックヘッダー }).childs([
            this._トラック名要素,
            div({ class: トラック属性群 }).childs([
                this._種類バッジ,
                this._楽器バッジ,
                this._音量バッジ,
                this._進行バッジ,
            ]),
        ])
    }

    public 表示を更新する(トラック: トラック定義): void {
        this._トラック名要素.clearChildren().child(span({ text: トラック.表示名 }))
        this._種類バッジ.clearChildren().child(span({ text: トラック.種類 }))
        this._楽器バッジ.clearChildren().child(span({ text: `楽器: ${トラック.楽器}` }))
        this._音量バッジ.clearChildren().child(span({ text: `音量: ${Math.round(トラック.音量 * 100)}%` }))
        this._進行バッジ.clearChildren().child(span({ text: トラック見出し部品.進行割り当て文言(トラック) }))
    }

    private static 進行割り当て文言(トラック: トラック定義): string {
        const 割り当て = トラック.進行の割り当て
        if (割り当て === null) {
            return '進行: パターン準拠'
        }
        if (割り当て.種類 === '既定の進行') {
            return `進行: 既定 (${割り当て.識別子})`
        }
        return `進行: 独自 (${割り当て.名前})`
    }
}
