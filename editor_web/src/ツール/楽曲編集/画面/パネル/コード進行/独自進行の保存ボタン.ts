import { ButtonC } from 'sengen-ui'
import { 主ボタン } from '../共通/スタイル.css.ts'

// 編集中の独自進行を保存するボタン。名前が空、または和音が1つも無い進行は保存できないため、
// 押しても何も起きない状態を作らず、押せない状態で出す。
export class 独自進行の保存ボタン extends ButtonC {
    public constructor() {
        super({ class: 主ボタン, text: '独自進行を保存', disabled: true })
        this.setTooltip('編集中の独自進行を保存')
    }

    public 編集中の内容を反映する(名前: string, 和音の数: number): this {
        this.setDisabled(名前 === '' || 和音の数 === 0)
        return this
    }
}
