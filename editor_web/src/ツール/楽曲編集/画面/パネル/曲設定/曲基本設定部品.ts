import { div, span, textInput, DivC, TextInputC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import {
    拍毎分の下限,
    拍毎分の上限,
    type 楽曲,
} from '../../../../../生成/編集資源契約.ts'
import { スライダー項目 } from '../共通/スライダー項目.ts'
import { 行コンテナ, 項目ラベル, テキスト入力 } from '../共通/スタイル.css.ts'

export interface I曲基本設定配線 {
    readonly on表示名変更: (新しい表示名: string) => void
    readonly on拍毎分変更: (新しい拍毎分: number) => void
}

// 楽曲の表示名とBPM（拍毎分）を入力する部品。
export class 曲基本設定部品 extends LV2HtmlComponentBase implements I配線可能<I曲基本設定配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I曲基本設定配線> = new 配線ポート<I曲基本設定配線>('曲基本設定部品')
    private readonly _表示名入力: TextInputC
    private readonly _拍毎分スライダー: スライダー項目

    public constructor(初期楽曲: 楽曲) {
        super()
        this._表示名入力 = textInput({ class: テキスト入力, value: 初期楽曲.表示名, placeholder: '楽曲の表示名' })
            .setTooltip('楽曲の表示名')
        this._拍毎分スライダー = new スライダー項目(
            '拍毎分 (BPM)',
            拍毎分の下限,
            拍毎分の上限,
            1,
            初期楽曲.拍毎分,
            ' BPM',
        )
        this._componentRoot = div().childs([
            div({ class: 行コンテナ }).childs([
                span({ class: 項目ラベル, text: '楽曲の表示名' }),
                this._表示名入力,
            ]),
            this._拍毎分スライダー,
        ])
    }

    public 配線する(配線: I曲基本設定配線): this {
        this._配線.配線する(配線)
        this._表示名入力.onChange(() => {
            if (this._配線.配線済みか) {
                this._配線.先.on表示名変更(this._表示名入力.getValue())
            }
        })
        this._拍毎分スライダー.配線する({
            on値変更: (新Bpm) => {
                if (this._配線.配線済みか) {
                    this._配線.先.on拍毎分変更(Math.round(新Bpm))
                }
            },
        })
        return this
    }

    public 表示を更新する(楽曲: 楽曲): void {
        this._表示名入力.setValue(楽曲.表示名)
        this._拍毎分スライダー.値を設定する(楽曲.拍毎分)
    }

    public override delete(): void {
        this._表示名入力.delete()
        this._拍毎分スライダー.delete()
        super.delete()
    }
}
