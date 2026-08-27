import { SelectC } from 'sengen-ui'
import type { コード進行, コード進行参照, トラック定義 } from '../../../../../生成/編集資源契約.ts'
import { 選択セレクト } from '../共通/スタイル.css.ts'
import {
    トラックの進行の選択肢一覧を組み立てる,
    綴りから進行の選びを復元する,
} from '../共通/コード進行選択肢.ts'

// トラックへ割り当てるコード進行を選ぶ欄。割り当てが無い状態(楽曲全体の進行に従う)も選択肢として出す。
export class トラックの進行選択欄 extends SelectC {
    public constructor(トラック: トラック定義, 独自進行一覧: readonly コード進行[]) {
        super({ class: 選択セレクト })
        this.トラックを反映する(トラック, 独自進行一覧)
    }

    public トラックを反映する(トラック: トラック定義, 独自進行一覧: readonly コード進行[]): this {
        this.setOptions(トラックの進行の選択肢一覧を組み立てる(トラック.進行の割り当て, 独自進行一覧))
        return this
    }

    // 割り当てが無いことは型契約でnullとして表されるため、楽曲全体に従う選びをnullへ写す。
    public 選ばれた進行の割り当て(): コード進行参照 | null {
        const 選び = 綴りから進行の選びを復元する(this.getValue())
        switch (選び.種類) {
            case '楽曲全体の進行に従う':
                return null
            case 'この進行を使う':
                return 選び.参照
        }
    }
}
