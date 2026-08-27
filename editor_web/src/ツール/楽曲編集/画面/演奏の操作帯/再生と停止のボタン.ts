import { ButtonC } from 'sengen-ui'
import { 主ボタン } from '../パネル/共通/スタイル.css.ts'

// 再生と停止を1つで兼ねるボタン。押した結果がその場で文字に出るよう、鳴っているかを文字が持つ。
export class 再生と停止のボタン extends ButtonC {
    private _映している再生中か: boolean = false

    public constructor() {
        super({ class: 主ボタン, text: '再生' })
        this.setTooltip('再生と停止を切り替える (鍵盤の空白でも切り替わる)')
    }

    // 再生中は画面の1コマごとに呼ばれるため、文字が変わるときだけ書き換える。
    public 再生中かを反映する(再生中か: boolean): this {
        if (this._映している再生中か === 再生中か) return this
        this._映している再生中か = 再生中か
        this.setTextContent(再生中か ? '停止' : '再生')
        return this
    }
}
