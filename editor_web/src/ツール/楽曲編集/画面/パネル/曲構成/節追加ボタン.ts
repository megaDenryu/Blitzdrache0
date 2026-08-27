import { ButtonC } from 'sengen-ui'
import { 主ボタン } from '../共通/スタイル.css.ts'
import { 追加ボタン } from './スタイル.css.ts'

// 曲構成の末尾へ節を足すボタン。足す節が指すのは選択中のパターンであるため、
// パターンが選ばれていないときは押しても何も起きない状態を作らず、押せない状態で出す。
export class 節追加ボタン extends ButtonC {
    public constructor() {
        super({ class: `${主ボタン} ${追加ボタン}`, text: '+ 節を追加', disabled: true })
        this.setTooltip('選択中のパターンを曲構成の末尾へ足す')
    }

    public 選択中パターンを反映する(選択中パターン名乗り: string | null): this {
        this.setDisabled(選択中パターン名乗り === null)
        return this
    }
}
