import { ButtonC } from 'sengen-ui'
import { 副ボタン } from '../パネル/共通/スタイル.css.ts'
import { メトロノームの切替 } from './スタイル.css.ts'

// メトロノームの入と切を1つで兼ねるボタン。押した結果がその場で見えるよう、入っているかを文字と枠の色が持つ。
export class メトロノームの切替ボタン extends ButtonC {
    private _映している入っているか: boolean | null = null

    public constructor() {
        super({ class: 副ボタン })
        this.addClass(メトロノームの切替)
        this.setTooltip('再生中の拍の頭で音を鳴らす。小節の頭は他の拍と違う音になる')
        this.入っているかを反映する(false)
    }

    // 演奏の様子を映すたびに呼ばれるため、文字が変わるときだけ書き換える。
    public 入っているかを反映する(入っているか: boolean): this {
        if (this._映している入っているか === 入っているか) return this
        this._映している入っているか = 入っているか
        this.setTextContent(入っているか ? 'メトロノーム 入' : 'メトロノーム 切')
        this.setAttribute('data-on', String(入っているか))
        return this
    }
}
