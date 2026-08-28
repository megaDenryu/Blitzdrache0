import { DivC } from 'sengen-ui'
import type { 升目の座標 } from '../../../../生成/編集資源契約.ts'
import { 選んでいる升目の札 as 選んでいる升目の札のスタイル } from '../スタイル.css.ts'

// いま選んでいる升目の位置を綴りで出す札。三次元で触った升目と平面図で編んでいる升目が同じであることを、
// 色の枠だけでなく綴りでも読めるようにする。
// 文言の差し替えを外から続けて行うため、裸のDivCを保持せずLV1拡張にする(SengenUIガイド第3条)。
export class 選んでいる升目の札 extends DivC {
    public constructor() {
        super({ class: 選んでいる升目の札のスタイル })
        this.表示を更新する(undefined)
    }

    public 表示を更新する(選んでいる升目: 升目の座標 | undefined): void {
        const 文言 = 選んでいる升目 === undefined
            ? '升目を選んでいない'
            : `選んでいる升目: 横${選んでいる升目.横}・奥${選んでいる升目.奥}・階${選んでいる升目.階}`
        this.setTextContent(文言).setTooltip(文言)
    }
}
