import { SpanC, 表示切替 } from 'sengen-ui'
import { 繰り返し中の印 } from './スタイル.css.ts'

// 選択中のカードが、パターンの繰り返しで鳴っているときにだけ出す小さな印。
export class 繰り返し中バッジ extends SpanC {
    public constructor() {
        super({ class: 繰り返し中の印, text: '繰り返し中' })
        this.toggleAttribute(表示切替.attribute, true, 表示切替.value.hidden)
    }

    public 繰り返し中かを示す(繰り返し中か: boolean): this {
        this.toggleAttribute(表示切替.attribute, !繰り返し中か, 表示切替.value.hidden)
        return this
    }
}
