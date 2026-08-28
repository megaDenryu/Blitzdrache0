import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { 全側面, type 升目の側面 } from '../../編集モデル/index.ts'
import { セクション, セクション見出し, 横並び, 選択ボタン } from '../スタイル.css.ts'

export interface I入口の向き配線 {
    readonly on入口の向く面を選ぶ: (側面: 升目の側面) => void
}

// 建物の入口がどの面を向くかを選ぶパネル。選ばせるのを4つの面に限るのは、焼く側が入口の方向を
// 建物を道へ向けるために使い、斜めや零の向きが意味を持たないためである。
export class 入口の向きパネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _列: DivC = div({ class: 横並び })

    public constructor() {
        super()
        this._componentRoot = div({ class: セクション }).childs([
            div({ class: セクション見出し, text: '入口の向く面' }),
            this._列,
        ])
    }

    // 保存物の向きが4つの面のどれとも一致しないときは、どのボタンも選ばれていない姿で出す。
    // 選ばれていない姿を黙って既定の面へ倒すと、人が触っていない向きが触ったことになる。
    public 再構築する(選んだ側面: 升目の側面 | undefined, 配線: I入口の向き配線): void {
        this._列.clearChildren()
        for (const 側面 of 全側面) {
            this._列.child(
                div({ class: 選択ボタン, text: 側面 })
                    .setTooltip(`入口を${側面}へ向ける`)
                    .setAttribute('data-selected', String(側面 === 選んだ側面))
                    .onClick(() => 配線.on入口の向く面を選ぶ(側面)),
            )
        }
    }
}
