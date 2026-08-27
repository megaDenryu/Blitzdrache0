import { SelectC } from 'sengen-ui'
import type { 曲の節, パターン } from '../../../../../生成/編集資源契約.ts'
import { 選択セレクト, 幅を伸ばすセレクト } from '../共通/スタイル.css.ts'
import { 節のパターン選択肢一覧を組み立てる } from './節の欄の値.ts'

// 曲構成の節が指すパターンを選ぶ欄。
export class 節のパターン選択欄 extends SelectC {
    public constructor(節: 曲の節, パターン一覧: readonly パターン[]) {
        super({ class: `${選択セレクト} ${幅を伸ばすセレクト}` })
        this.setOptions(節のパターン選択肢一覧を組み立てる(節, パターン一覧))
    }

    public 選ばれたパターンの名乗り(): string {
        return this.getValue()
    }
}
