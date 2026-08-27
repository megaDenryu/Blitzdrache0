import { SelectC } from 'sengen-ui'
import type { パターン } from '../../../../../生成/編集資源契約.ts'
import { 選択セレクト, 幅を伸ばすセレクト } from '../共通/スタイル.css.ts'

// 編集の対象にするパターンを選ぶ欄。一覧の表示の綴りと、選ばれた名乗りの取り出しを持つ。
export class パターン選択欄 extends SelectC {
    public constructor(パターン一覧: readonly パターン[], 選ばれている名乗り: string | null) {
        super({ class: `${選択セレクト} ${幅を伸ばすセレクト}` })
        this.パターン一覧を反映する(パターン一覧, 選ばれている名乗り)
    }

    public パターン一覧を反映する(パターン一覧: readonly パターン[], 選ばれている名乗り: string | null): this {
        this.setOptions(パターン一覧.map((パターン) => ({
            value: パターン.名乗り,
            text: `${パターン.表示名} (${パターン.名乗り})`,
            selected: パターン.名乗り === 選ばれている名乗り,
        })))
        return this
    }

    public 選ばれた名乗り(): string | null {
        const 名乗り = this.getValue()
        return 名乗り === '' ? null : 名乗り
    }
}
