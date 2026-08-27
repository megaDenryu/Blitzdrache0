import { SpanC } from 'sengen-ui'
import type { 楽曲 } from '../../../../../生成/編集資源契約.ts'
import { 案内の帯 } from '../共通/スタイル.css.ts'
import { 曲構成の概要表示文を組み立てる } from './曲構成表示計算.ts'

// 曲全体の長さと節の並びを1行で見せる帯。曲構成が空のときに、いま開いているパターンだけを
// 繰り返して鳴らすことをここで伝える。
export class 曲構成の概要表示 extends SpanC {
    public constructor() {
        super({ class: 案内の帯 })
    }

    public 曲構成を反映する(楽曲: 楽曲, 選択中パターン名乗り: string | null): this {
        this.setTextContent(曲構成の概要表示文を組み立てる(楽曲.曲構成, 楽曲.パターン一覧, 選択中パターン名乗り))
        return this
    }
}
