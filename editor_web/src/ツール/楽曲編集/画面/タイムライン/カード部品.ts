import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { パターン } from '../../../../生成/編集資源契約.ts'
import type { カード位置, 曲構成のカード } from '../../編集モデル/index.ts'
import { パターンの表示名を探す } from './パターンの表示名を探す.ts'
import { 繰り返し中バッジ } from './繰り返し中バッジ.ts'
import { カード枠, カードのパターン名, カードの小節番号 } from './スタイル.css.ts'

export interface Iカード配線 {
    readonly onクリック: () => void
}

// タイムラインの1枚のカードは曲全体の中の1小節を表す(判断16、issue #87)。
// パターンの表示名と曲全体での小節番号を表示し、選択だけを受け持つ。
// 削除・複製・前後挿入・前後移動の操作は、同じパターンのカードを括る節の枠のボタンへ集めた。
// 再生位置と繰り返し中の印は毎フレーム別経路で呼ばれるため、
// 変わったときだけ属性を書き換える(打ち込み升目部品・再生位置の表示と同じ規律)。
export class カード部品 extends LV2HtmlComponentBase implements I配線可能<Iカード配線> {
    protected _componentRoot: DivC
    public readonly 位置: カード位置
    private readonly _配線: 配線ポート<Iカード配線> = new 配線ポート<Iカード配線>('カード部品')
    private readonly _繰り返し中バッジ: 繰り返し中バッジ = new 繰り返し中バッジ()

    public constructor(
        カード: 曲構成のカード,
        パターン一覧: readonly パターン[],
        選択中か: boolean,
    ) {
        super()
        this.位置 = カード.位置
        this._componentRoot = div({ class: カード枠 }).childs([
            div({ class: カードのパターン名, text: パターンの表示名を探す(カード.パターンの名乗り, パターン一覧) }),
            span({ class: カードの小節番号, text: String(カード.曲全体での小節番号) }),
            this._繰り返し中バッジ,
        ])
        this._componentRoot.setAttribute('data-節偶奇', String(((カード.位置.節の位置 % 2) + 2) % 2))
        this._componentRoot.setAttribute('data-選択中', String(選択中か))
        this._componentRoot.setAttribute('data-再生中', 'false')
        this._componentRoot.addTypedEventListener('click', () => {
            if (this._配線.配線済みか) this._配線.先.onクリック()
        })
    }

    public 配線する(配線: Iカード配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 再生中の印を示す(いま鳴っているか: boolean): void {
        this._componentRoot.setAttribute('data-再生中', String(いま鳴っているか))
    }

    public 繰り返し中の印を示す(繰り返し中か: boolean): void {
        this._繰り返し中バッジ.繰り返し中かを示す(繰り返し中か)
    }

    public override delete(): void {
        this._繰り返し中バッジ.delete()
        super.delete()
    }
}
