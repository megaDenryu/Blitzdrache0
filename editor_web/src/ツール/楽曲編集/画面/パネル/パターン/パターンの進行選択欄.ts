import { SelectC } from 'sengen-ui'
import type { コード進行, コード進行参照, パターン } from '../../../../../生成/編集資源契約.ts'
import { 選択セレクト } from '../共通/スタイル.css.ts'
import {
    パターンの進行の選択肢一覧を組み立てる,
    綴りから進行の選びを復元する,
} from '../共通/コード進行選択肢.ts'

// パターンへ割り当てるコード進行を選ぶ欄。パターンは必ず進行を1つ持つため、割り当て無しは選べない。
export class パターンの進行選択欄 extends SelectC {
    public constructor() {
        super({ class: 選択セレクト })
    }

    public パターンを反映する(パターン: パターン | null, 独自進行一覧: readonly コード進行[]): this {
        this.setDisabled(パターン === null)
        this.setOptions(
            パターン === null ? [] : パターンの進行の選択肢一覧を組み立てる(パターン.進行の参照, 独自進行一覧),
        )
        return this
    }

    // 割り当て無しの選択肢を出していないため、それが選ばれていたら画面の配線の誤りである。
    public 選ばれた進行の参照(): コード進行参照 {
        const 選び = 綴りから進行の選びを復元する(this.getValue())
        if (選び.種類 === '楽曲全体の進行に従う') {
            throw new Error('パターンの進行に、楽曲全体に従う選びが渡されました')
        }
        return 選び.参照
    }
}
