import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { パターン } from '../../../../生成/編集資源契約.ts'
import type { カード位置, 曲構成のカード } from '../../編集モデル/index.ts'
import { カードの操作ボタン群, type カード操作の押せるか } from './カードの操作ボタン群.ts'
import type { カード操作の種類 } from './カード操作の種類.ts'
import { 繰り返し中バッジ } from './繰り返し中バッジ.ts'
import { カード枠, カードのパターン名, カードの小節番号 } from './スタイル.css.ts'

export interface Iカード配線 {
    readonly onクリック: () => void
    readonly on操作: (種類: カード操作の種類) => void
}

// タイムラインの1枚のカード。パターンの表示名と小節番号の範囲を表示し、
// 選択中のときだけ操作ボタン群を出す。再生位置と繰り返し中の印は毎フレーム別経路で呼ばれるため、
// 変わったときだけ属性を書き換える(打ち込み升目部品・再生位置の表示と同じ規律)。
export class カード部品 extends LV2HtmlComponentBase implements I配線可能<Iカード配線> {
    protected _componentRoot: DivC
    public readonly 位置: カード位置
    private readonly _配線: 配線ポート<Iカード配線> = new 配線ポート<Iカード配線>('カード部品')
    private readonly _繰り返し中バッジ: 繰り返し中バッジ = new 繰り返し中バッジ()
    private readonly _ボタン群: カードの操作ボタン群 | null

    public constructor(
        カード: 曲構成のカード,
        パターン一覧: readonly パターン[],
        選択中か: boolean,
        押せるか: カード操作の押せるか | null,
    ) {
        super()
        this.位置 = カード.位置
        this._ボタン群 = 選択中か && 押せるか !== null ? new カードの操作ボタン群(押せるか) : null
        this._componentRoot = div({ class: カード枠 }).childs([
            div({ class: カードのパターン名, text: カード部品._表示名を探す(カード, パターン一覧) }),
            span({
                class: カードの小節番号,
                text: `${カード.小節番号の範囲.始まりの小節番号}-${カード.小節番号の範囲.終わりの小節番号}`,
            }),
            this._繰り返し中バッジ,
            ...(this._ボタン群 === null ? [] : [this._ボタン群]),
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
        this._ボタン群?.配線する({ on操作: (種類) => 配線.on操作(種類) })
        return this
    }

    public 再生中の印を示す(いま鳴っているか: boolean): void {
        this._componentRoot.setAttribute('data-再生中', String(いま鳴っているか))
    }

    public 繰り返し中の印を示す(繰り返し中か: boolean): void {
        this._繰り返し中バッジ.繰り返し中かを示す(繰り返し中か)
    }

    public override delete(): void {
        this._ボタン群?.delete()
        this._繰り返し中バッジ.delete()
        super.delete()
    }

    private static _表示名を探す(カード: 曲構成のカード, パターン一覧: readonly パターン[]): string {
        const パターン = パターン一覧.find((p) => p.名乗り === カード.パターンの名乗り)
        return パターン !== undefined ? パターン.表示名 : カード.パターンの名乗り
    }
}
