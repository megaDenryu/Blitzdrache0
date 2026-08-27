import { SelectC } from 'sengen-ui'
import type { 曲の節 } from '../../../../../生成/編集資源契約.ts'
import { 選択セレクト } from '../共通/スタイル.css.ts'
import { 回数選択 } from './スタイル.css.ts'
import { 節の繰り返し回数の選択肢一覧を組み立てる, 綴りから繰り返し回数を復元する } from './節の欄の値.ts'

// 曲構成の節がパターンを何回繰り返すかを選ぶ欄。
export class 節の繰り返し回数選択欄 extends SelectC {
    public constructor(節: 曲の節) {
        super({ class: `${選択セレクト} ${回数選択}` })
        this.setOptions(節の繰り返し回数の選択肢一覧を組み立てる(節))
    }

    public 選ばれた繰り返し回数(): number {
        return 綴りから繰り返し回数を復元する(this.getValue())
    }
}
