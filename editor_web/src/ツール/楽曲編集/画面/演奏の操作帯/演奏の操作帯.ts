import { button, div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲 } from '../../../../生成/編集資源契約.ts'
import type { 演奏の範囲 } from '../../編集モデル/index.ts'
import type { 再生位置, 演奏の知らせ } from '../演奏/index.ts'
import { 副ボタン, 危険ボタン } from '../パネル/共通/スタイル.css.ts'
import { 再生と停止のボタン } from './再生と停止のボタン.ts'
import { 再生位置の表示 } from './再生位置の表示.ts'
import { テンポの欄 } from './テンポの欄.ts'
import { 演奏の範囲選択欄 } from './演奏の範囲選択欄.ts'
import { 演奏の知らせの表示 } from './演奏の知らせの表示.ts'
import { 操作帯の行, 操作帯枠 } from './スタイル.css.ts'

export interface I演奏の操作帯配線 {
    readonly on再生と停止: () => void
    readonly on先頭へ戻す: () => void
    readonly on演奏の範囲変更: (範囲: 演奏の範囲) => void
    readonly onテンポ変更: (新しいテンポ: number) => void
    readonly on全消去: () => void
    readonly on見本の曲: () => void
}

// 格子の上に常設する演奏の操作帯。再生と停止・先頭へ戻す・再生位置・演奏の範囲・テンポと、
// 打ち込みをまとめて置き換える2つの操作(全消去・見本の曲)を持つ。
export class 演奏の操作帯 extends LV2HtmlComponentBase implements I配線可能<I演奏の操作帯配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I演奏の操作帯配線> = new 配線ポート<I演奏の操作帯配線>('演奏の操作帯')
    private readonly _再生と停止: 再生と停止のボタン = new 再生と停止のボタン()
    private readonly _再生位置: 再生位置の表示 = new 再生位置の表示()
    private readonly _知らせ: 演奏の知らせの表示 = new 演奏の知らせの表示()
    private readonly _範囲選択: 演奏の範囲選択欄
    private readonly _テンポ: テンポの欄

    public constructor(初期楽曲: 楽曲, 初期の範囲: 演奏の範囲) {
        super()
        this._範囲選択 = new 演奏の範囲選択欄(初期の範囲)
        this._テンポ = new テンポの欄(初期楽曲.テンポ)
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I演奏の操作帯配線): this {
        this._配線.配線する(配線)
        this._テンポ.配線する({ onテンポ変更: (値) => 配線.onテンポ変更(値) })
        this._範囲選択.onSelectChange(() => 配線.on演奏の範囲変更(this._範囲選択.選ばれた範囲()))
        return this
    }

    public 楽曲を反映する(楽曲: 楽曲): void {
        this._テンポ.値を設定する(楽曲.テンポ)
    }

    public 演奏の様子を反映する(
        再生中か: boolean,
        範囲: 演奏の範囲,
        位置: 再生位置 | null,
        パターンの表示名: string | null,
    ): void {
        this._再生と停止.再生中かを反映する(再生中か)
        this._範囲選択.範囲を反映する(範囲)
        this._再生位置.再生位置を反映する(位置, パターンの表示名)
    }

    // 音を出せなかったことを帯に出す。押した結果が画面に何も出ない状態を作らないための口である。
    public 演奏の知らせを反映する(知らせ: 演奏の知らせ | null): void {
        this._知らせ.知らせを反映する(知らせ)
    }

    public override delete(): void {
        this._知らせ.delete()
        this._再生と停止.delete()
        this._再生位置.delete()
        this._範囲選択.delete()
        this._テンポ.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return div({ class: 操作帯枠 }).childs([
            div({ class: 操作帯の行 }).childs([
                this._再生と停止.onClick(() => this._配線.先.on再生と停止()),
                button({ class: 副ボタン, text: '先頭へ戻す' })
                    .setTooltip('再生位置を曲の先頭へ戻す')
                    .onClick(() => this._配線.先.on先頭へ戻す()),
                this._再生位置,
                this._範囲選択,
                this._知らせ,
            ]),
            div({ class: 操作帯の行 }).childs([
                this._テンポ,
                button({ class: 副ボタン, text: '見本の曲を入れる' })
                    .setTooltip('試作と同じ見本の打ち込みを、いま見ているパターンへ入れる')
                    .onClick(() => this._配線.先.on見本の曲()),
                button({ class: 危険ボタン, text: '全消去' })
                    .setTooltip('いま見ているパターンの打ち込みを全部消す')
                    .onClick(() => this._配線.先.on全消去()),
            ]),
        ])
    }
}
