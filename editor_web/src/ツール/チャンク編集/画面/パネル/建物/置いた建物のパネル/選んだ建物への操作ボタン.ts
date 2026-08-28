import { ButtonC } from 'sengen-ui'

// 建物を選んでいるときだけ押せる操作のボタン(LV1拡張)。選んでいないときに押しても地形も建物も
// 変わらないため、押せない見た目にして「押しても画面が変わらない操作」を残さない
// (エディター制作スキル第5条)。
export class 選んだ建物への操作ボタン extends ButtonC {
    public constructor(見出し: string, クラス: string) {
        super({ class: クラス, text: 見出し, disabled: true })
        this.setTooltip(見出し)
    }

    public 有効状態を設定する(有効: boolean): this {
        this.setDisabled(!有効)
        return this
    }
}
