import { NumberInputC } from 'sengen-ui'
import { パターンの小節数の上限, type パターン } from '../../../../../生成/編集資源契約.ts'
import { テキスト入力 } from '../共通/スタイル.css.ts'

// 選択中パターンの小節数を1から上限まで選ぶ欄。パターンが選ばれていないときは書き込めない。
// 減らすと末尾の小節の打点が消えることを、押しどころのtooltipで伝える。
export class パターン小節数欄 extends NumberInputC {
    public constructor() {
        super({ class: テキスト入力, min: 1, max: パターンの小節数の上限, step: 1 })
        this.setTooltip('パターンの小節数(減らすと末尾の小節の打点が消える)')
    }

    public パターンを反映する(パターン: パターン | null): this {
        this.setValue(パターン === null ? 1 : パターン.小節数)
        this.setDisabled(パターン === null)
        return this
    }
}
