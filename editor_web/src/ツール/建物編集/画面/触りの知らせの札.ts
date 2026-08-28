import { span, DivC } from 'sengen-ui'
import { 断りの文言, 触りの知らせ } from './スタイル.css.ts'

// 筆が断られた事情を平面図の下へ出す札。断りを黙って捨てると、触ったのに何も起きない理由が人に読めない。
// 文言の差し替えを外から続けて行うため、裸のDivCを保持せずLV1拡張として自分でドメインのメソッドを持つ
// (SengenUIガイド第3条)。
export class 触りの知らせの札 extends DivC {
    public constructor() {
        super({ class: 触りの知らせ })
    }

    public 文言を示す(文言: string): void {
        this.setTooltip(文言).clearChildren()
        if (文言 !== '') this.child(span({ class: 断りの文言, text: 文言 }))
    }
}
