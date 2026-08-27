import { SelectC } from 'sengen-ui'
import { 演奏の範囲の一覧, 演奏の範囲の表示名, type 演奏の範囲 } from '../../編集モデル/index.ts'
import { 範囲の選択 } from './スタイル.css.ts'

// パターンだけを繰り返すか、曲構成のとおりに鳴らすかを選ぶ欄。
export class 演奏の範囲選択欄 extends SelectC {
    private _映している範囲: 演奏の範囲 | null = null

    public constructor(初期の範囲: 演奏の範囲) {
        super({ class: 範囲の選択 })
        this.setTooltip('再生したときに何が鳴るかを選ぶ')
        this.範囲を反映する(初期の範囲)
    }

    // 再生中は画面の1コマごとに呼ばれるため、選び直しが要るときだけ選択肢を組み直す。
    public 範囲を反映する(範囲: 演奏の範囲): this {
        if (this._映している範囲 === 範囲) return this
        this._映している範囲 = 範囲
        this.setOptions(
            演奏の範囲の一覧.map((候補) => ({
                value: 候補,
                text: 演奏の範囲の表示名(候補),
                selected: 候補 === 範囲,
            })),
        )
        return this
    }

    // 選ばれた綴りはこの欄が出した選択肢のものしか来ないため、外れた綴りは配線の誤りとして失敗させる。
    public 選ばれた範囲(): 演奏の範囲 {
        const 綴り = this.getValue()
        for (const 候補 of 演奏の範囲の一覧) {
            if (候補 === 綴り) return 候補
        }
        throw new Error(`演奏の範囲として選べない綴りが選ばれています: ${綴り}`)
    }
}
