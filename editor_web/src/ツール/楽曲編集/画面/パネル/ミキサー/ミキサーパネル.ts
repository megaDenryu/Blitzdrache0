import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲, ミキサー設定 } from '../../../../../生成/編集資源契約.ts'
import { ミキサー設定部品 } from './ミキサー設定部品.ts'
import { パネル外枠, パネル見出し } from '../共通/スタイル.css.ts'

export interface Iミキサーパネル配線 {
    readonly onミキサー設定変更: (新しいミキサー設定: ミキサー設定) => void
}

// 楽曲全体の音量と効果を調整するパネル。
// 楽曲の表示名は格子の上の固定の帯が持つ。同じ値を2箇所で変えられる形にしないため、この枠からは外してある。
export class ミキサーパネル extends LV2HtmlComponentBase implements I配線可能<Iミキサーパネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iミキサーパネル配線> = new 配線ポート<Iミキサーパネル配線>('ミキサーパネル')
    private readonly _ミキサー設定: ミキサー設定部品

    public constructor(初期楽曲: 楽曲) {
        super()
        this._ミキサー設定 = new ミキサー設定部品(初期楽曲)
        this._componentRoot = div({ class: パネル外枠 }).childs([
            div({ class: パネル見出し, text: 'ミキサーの設定' }),
            this._ミキサー設定,
        ])
    }

    public 配線する(配線: Iミキサーパネル配線): this {
        this._配線.配線する(配線)
        this._ミキサー設定.配線する({
            onミキサー設定変更: (設定) => {
                if (this._配線.配線済みか) this._配線.先.onミキサー設定変更(設定)
            },
        })
        return this
    }

    public 表示を更新する(楽曲: 楽曲): void {
        this._ミキサー設定.表示を更新する(楽曲)
    }

    public override delete(): void {
        this._ミキサー設定.delete()
        super.delete()
    }
}
