import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲, ミキサー設定 } from '../../../../../生成/編集資源契約.ts'
import { 曲基本設定部品 } from './曲基本設定部品.ts'
import { ミキサー設定部品 } from './ミキサー設定部品.ts'
import { パネル外枠, パネル見出し } from '../共通/スタイル.css.ts'

export interface I曲設定パネル配線 {
    readonly on表示名変更: (新しい表示名: string) => void
    readonly on拍毎分変更: (新しい拍毎分: number) => void
    readonly onミキサー設定変更: (新しいミキサー設定: ミキサー設定) => void
}

// 楽曲全体の基本情報（表示名・BPM）およびミキサー設定を編集するパネル。
export class 曲設定パネル extends LV2HtmlComponentBase implements I配線可能<I曲設定パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I曲設定パネル配線> = new 配線ポート<I曲設定パネル配線>('曲設定パネル')
    private readonly _基本設定: 曲基本設定部品
    private readonly _ミキサー設定: ミキサー設定部品

    public constructor(初期楽曲: 楽曲) {
        super()
        this._基本設定 = new 曲基本設定部品(初期楽曲)
        this._ミキサー設定 = new ミキサー設定部品(初期楽曲)
        this._componentRoot = div({ class: パネル外枠 }).childs([
            div({ class: パネル見出し, text: '楽曲とミキサーの設定' }),
            this._基本設定,
            this._ミキサー設定,
        ])
    }

    public 配線する(配線: I曲設定パネル配線): this {
        this._配線.配線する(配線)
        this._基本設定.配線する({
            on表示名変更: (名) => {
                if (this._配線.配線済みか) this._配線.先.on表示名変更(名)
            },
            on拍毎分変更: (bpm) => {
                if (this._配線.配線済みか) this._配線.先.on拍毎分変更(bpm)
            },
        })
        this._ミキサー設定.配線する({
            onミキサー設定変更: (設定) => {
                if (this._配線.配線済みか) this._配線.先.onミキサー設定変更(設定)
            },
        })
        return this
    }

    public 表示を更新する(楽曲: 楽曲): void {
        this._基本設定.表示を更新する(楽曲)
        this._ミキサー設定.表示を更新する(楽曲)
    }

    public override delete(): void {
        this._基本設定.delete()
        this._ミキサー設定.delete()
        super.delete()
    }
}
