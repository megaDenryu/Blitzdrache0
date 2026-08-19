import { ButtonC } from 'sengen-ui'
import { 削除ボタン } from '../スタイル.css.ts'

// 建物パネルの削除操作ボタン。選択建物の有無に応じた有効/無効の切り替えだけを持つ。
export class 建物削除ボタン extends ButtonC {
    public constructor() {
        super({ class: 削除ボタン, text: '削除', disabled: true })
        this.setTooltip('削除')
    }

    public 有効状態を設定する(有効: boolean): this {
        this.setDisabled(!有効)
        return this
    }
}
