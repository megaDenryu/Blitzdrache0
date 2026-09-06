import { ButtonC } from 'sengen-ui'
import { 主ボタン } from '../パネル/共通/スタイル.css.ts'
import { 末尾へ追加ボタンの配置 } from './スタイル.css.ts'

// 選択中のパターンをタイムラインの末尾へ追加するボタン。パターンが選ばれていないときは押せない状態で出す。
export class 末尾へ追加ボタン extends ButtonC {
    public constructor() {
        super({ class: `${主ボタン} ${末尾へ追加ボタンの配置}`, text: '+ 末尾へ追加' })
        this.setTooltip('選択中のパターンをタイムラインの末尾へ追加')
    }

    public 選択中パターンを反映する(選択中パターンの名乗り: string | null): this {
        this.setDisabled(選択中パターンの名乗り === null)
        return this
    }
}
