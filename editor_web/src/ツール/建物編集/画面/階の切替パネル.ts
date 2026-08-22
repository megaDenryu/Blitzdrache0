import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { セクション, セクション見出し, 横並び, 選択ボタン } from './スタイル.css.ts'

export interface I階の切替配線 {
    readonly on階を選ぶ: (階: number) => void
}

// 何階を平面図に出すかの切替。既にある階に加えて1つ上の階を必ず出すのは、階を積む操作の入口が
// 画面に無いと、人が2階を作れないためである。
export class 階の切替パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _列: DivC = div({ class: 横並び })

    public constructor() {
        super()
        this._componentRoot = div({ class: セクション }).childs([
            div({ class: セクション見出し, text: '階' }),
            this._列,
        ])
    }

    public 再構築する(最上階: number, 選んだ階: number, 配線: I階の切替配線): void {
        this._列.clearChildren()
        for (let 階 = 0; 階 <= 最上階 + 1; 階 += 1) {
            const この階 = 階
            this._列.child(
                div({ class: 選択ボタン, text: `${階}階` })
                    .setTooltip(`${階}階`)
                    .setAttribute('data-selected', String(階 === 選んだ階))
                    .onClick(() => 配線.on階を選ぶ(この階)),
            )
        }
    }
}
