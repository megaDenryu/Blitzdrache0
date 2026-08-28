import { ButtonC } from 'sengen-ui'
import { 操作ボタン } from './操作帯.css.ts'

// 直前の操作を取り消すボタン(LV1拡張)。押せるかどうかが取り消しの履歴の有無で変わるため、
// 見た目の更新をこのクラスへ閉じる。チャンク編集と大域編集の操作帯が共有する。
export class 取り消しボタン extends ButtonC {
    public constructor() {
        super({ class: 操作ボタン, text: '取り消し', disabled: true })
        this.setTooltip('直前の操作を取り消す(Ctrl+Z)')
    }

    public 押せるか設定する(押せるか: boolean): this {
        this.setDisabled(!押せるか)
        return this
    }
}
