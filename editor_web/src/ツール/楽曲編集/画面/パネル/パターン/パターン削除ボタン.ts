import { ButtonC } from 'sengen-ui'
import { 危険ボタン } from '../共通/スタイル.css.ts'
import { パターンを削除できるか } from './パターン操作判定.ts'

// 選択中パターンを削除するボタン。楽曲は少なくとも1つのパターンを持つため、
// 最後の1つのときは押せない状態で出す。
export class パターン削除ボタン extends ButtonC {
    public constructor() {
        super({ class: 危険ボタン, text: '削除', disabled: true })
        this.setTooltip('選択中パターンを削除')
    }

    public パターン数を反映する(パターン数: number): this {
        this.setDisabled(!パターンを削除できるか(パターン数))
        return this
    }
}
