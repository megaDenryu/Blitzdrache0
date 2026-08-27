import { TextInputC } from 'sengen-ui'
import type { パターン } from '../../../../../生成/編集資源契約.ts'
import { テキスト入力 } from '../共通/スタイル.css.ts'

// 選択中パターンの表示名を書き換える欄。パターンが選ばれていないときに書き込めないよう、
// 値の反映と入力の可否をこの欄が一体で持つ。
export class パターン表示名入力欄 extends TextInputC {
    public constructor() {
        super({ class: テキスト入力, placeholder: 'パターンの表示名' })
        this.setTooltip('パターンの表示名')
    }

    public パターンを反映する(パターン: パターン | null): this {
        this.setValue(パターン === null ? '' : パターン.表示名)
        this.setDisabled(パターン === null)
        return this
    }
}
