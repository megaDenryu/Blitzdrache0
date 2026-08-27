import { SelectC } from 'sengen-ui'
import type { トラック定義, 楽器 } from '../../../../../生成/編集資源契約.ts'
import { 選択セレクト } from '../共通/スタイル.css.ts'
import { トラックの種類から選べる楽器一覧を取得する, トラックの楽器は妥当か } from './トラック楽器選択肢.ts'

// トラックの楽器を選ぶ欄。トラックの種類で選べる楽器が変わるため、選択肢の絞り込みと
// 選ばれた綴りを楽器の型へ戻す判定をこの欄が持つ。
export class トラックの楽器選択欄 extends SelectC {
    private _トラックの種類: トラック定義['種類']

    public constructor(トラック: トラック定義) {
        super({ class: 選択セレクト })
        this._トラックの種類 = トラック.種類
        this.トラックを反映する(トラック)
    }

    public トラックを反映する(トラック: トラック定義): this {
        this._トラックの種類 = トラック.種類
        this.setOptions(
            トラックの種類から選べる楽器一覧を取得する(トラック.種類).map((楽器名) => ({
                value: 楽器名,
                text: 楽器名,
                selected: 楽器名 === トラック.楽器,
            })),
        )
        return this
    }

    // 選ばれた綴りはこの欄が出した選択肢のものしか来ないため、種類に合わない綴りは配線の誤りとして失敗させる。
    public 選ばれた楽器(): 楽器 {
        const 綴り = this.getValue()
        for (const 楽器名 of トラックの種類から選べる楽器一覧を取得する(this._トラックの種類)) {
            if (楽器名 === 綴り && トラックの楽器は妥当か(this._トラックの種類, 楽器名)) {
                return 楽器名
            }
        }
        throw new Error(`${this._トラックの種類}トラックで選べない楽器が選ばれています: ${綴り}`)
    }
}
