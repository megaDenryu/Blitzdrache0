import { div, span, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { セクション, セクション見出し, 選択ボタン, 階の1件, 階の一覧の巻き取り枠 } from '../スタイル.css.ts'

export interface I階の一覧配線 {
    readonly on階を選ぶ: (階: number) => void
}

// いま建物が持つ階を並べ、どの階を平面図へ出すかを選ばせる一覧。
// 既にある階に加えて1つ上の階を必ず出すのは、階を積む操作の入口が画面に無いと、人が2階を作れないためである。
// 右サイドバーへ置くのは、これが「いま選んでいるものの中の要素の一覧」だからである(設計正本の判断14)。
export class 階の一覧パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _一覧: DivC = div({ class: 階の一覧の巻き取り枠 })

    public constructor() {
        super()
        this._componentRoot = div({ class: セクション }).childs([
            div({ class: セクション見出し, text: '階' }),
            this._一覧,
        ])
    }

    public 再構築する(最上階: number, 選んだ階: number, 升目の数を数える: (階: number) => number, 配線: I階の一覧配線): void {
        this._一覧.clearChildren()
        for (let 階 = 0; 階 <= 最上階 + 1; 階 += 1) {
            const この階 = 階
            const 升目の数 = 升目の数を数える(階)
            this._一覧.child(
                div({ class: `${選択ボタン} ${階の1件}` })
                    .setTooltip(升目の数 === 0 ? `${階}階(まだ升目が無い)` : `${階}階(升目${升目の数}個)`)
                    .setAttribute('data-selected', String(階 === 選んだ階))
                    .onClick(() => 配線.on階を選ぶ(この階))
                    .childs([
                        span({ text: `${階}階` }),
                        span({ text: 升目の数 === 0 ? '空' : `升目${升目の数}` }),
                    ]),
            )
        }
    }
}
