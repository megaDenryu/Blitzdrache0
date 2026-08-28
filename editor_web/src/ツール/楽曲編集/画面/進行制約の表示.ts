import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { 情報バッジ } from './スタイル.css.ts'

function 進行制約の文言(進行の外モードか: boolean): string {
    return 進行の外モードか ? '進行制約: 進行の外' : '進行制約: 追従'
}

// 打ち込みが和音へ追従するか、和音の外の音も置けるかを示す札。
// 打ち込みの手が変わる情報のため、右サイドバーへ移さず和音の帯と同じ行に残す(設計正本の判断14)。
export class 進行制約の表示 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC

    public constructor() {
        super()
        this._componentRoot = div({ class: 情報バッジ, text: 進行制約の文言(false) })
            .setTooltip('打ち込みが和音へ追従するかどうか。Altを押すたびに入れ替わる')
    }

    public 表示を更新する(進行の外モードか: boolean): void {
        this._componentRoot.setTextContent(進行制約の文言(進行の外モードか))
    }
}
