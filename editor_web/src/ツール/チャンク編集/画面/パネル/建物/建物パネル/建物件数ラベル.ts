import { SpanC } from 'sengen-ui'
import { 件数ラベル } from '../スタイル.css.ts'

// 建物パネルの見出し行に置く件数表示。件数フィールドだけを持ち、更新はこの型のメソッドで行う。
export class 建物件数ラベル extends SpanC {
    public constructor(初期件数: number) {
        super({ class: 件数ラベル, text: `${初期件数} 件` })
    }

    public 件数を更新する(件数: number): this {
        this.setTextContent(`${件数} 件`)
        return this
    }
}
