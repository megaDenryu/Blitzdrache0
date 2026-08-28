import { div, span, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 道路の設定 } from '../../../編集モデル/index.ts'
import { スライダー項目 } from '../共通/スライダー項目.ts'
import { 道路の操作ボタン群, type I道路の操作ボタン群配線 } from '../共通/道路の操作ボタン群.ts'
import { パネル, 見出し, アクション区画, 切土盛土ボタン } from './スタイル.css.ts'

export interface I道路パネル配線 extends I道路の操作ボタン群配線 {
    readonly on全幅変更: (全幅: number) => void
    readonly on除外バッファ変更: (バッファ: number) => void
    readonly on細分割数変更: (細分割: number) => void
    readonly on道路切土盛土: () => void
}

// チャンクの道路の幅・散布除外バッファ・細分割数の設定と、道の追加・削除・切土盛土造成を
// 提供するLV2素部品。設定は「対象の道」1本にだけ効く(道は何本でも置ける)。
export class 道路パネル extends LV2HtmlComponentBase implements I配線可能<I道路パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I道路パネル配線> = new 配線ポート<I道路パネル配線>('道路パネル')
    private readonly _全幅スライダー: スライダー項目
    private readonly _除外バッファスライダー: スライダー項目
    private readonly _細分割スライダー: スライダー項目
    private readonly _操作ボタン群: 道路の操作ボタン群

    public constructor(初期全幅: number, 初期バッファ: number, 初期細分割: number) {
        super()
        this._全幅スライダー = new スライダー項目('道路全幅', 2, 24, 0.5, 初期全幅, 'm')
        this._除外バッファスライダー = new スライダー項目('散布除外バッファ', 4, 40, 1, 初期バッファ, 'm')
        this._細分割スライダー = new スライダー項目('細分割数', 20, 300, 10, 初期細分割)
        this._操作ボタン群 = new 道路の操作ボタン群()
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I道路パネル配線): this {
        this._配線.配線する(配線)
        this._全幅スライダー.配線する({ on値変更: (v) => this._配線.先.on全幅変更(v) })
        this._除外バッファスライダー.配線する({ on値変更: (v) => this._配線.先.on除外バッファ変更(v) })
        this._細分割スライダー.配線する({ on値変更: (v) => this._配線.先.on細分割数変更(v) })
        this._操作ボタン群.配線する(配線)
        return this
    }

    public 選択中の道路点があるか設定する(選択あり: boolean): void {
        this._操作ボタン群.選択中の道路点があるか設定する(選択あり)
    }

    // 設定が効く道を切り替える。設定がnullのときは対象の道が無く、スライダーを操作させない。
    public 対象の道を設定する(道路添字: number | null, 本数: number, 設定: 道路の設定 | null): void {
        this._操作ボタン群.対象の道を設定する(道路添字, 本数)
        const 操作できるか = 設定 !== null
        this._全幅スライダー.操作できるか設定する(操作できるか)
        this._除外バッファスライダー.操作できるか設定する(操作できるか)
        this._細分割スライダー.操作できるか設定する(操作できるか)
        if (設定 === null) return
        this._全幅スライダー.値を設定する(設定.全幅メートル)
        this._除外バッファスライダー.値を設定する(設定.散布除外バッファメートル)
        this._細分割スライダー.値を設定する(設定.細分割数)
    }

    public override delete(): void {
        this._全幅スライダー.delete()
        this._除外バッファスライダー.delete()
        this._細分割スライダー.delete()
        this._操作ボタン群.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                span({ class: 見出し, text: '選んでいる道の設定' }).setTooltip('選んでいる道の設定'),
                this._全幅スライダー,
                this._除外バッファスライダー,
                this._細分割スライダー,
                div({ class: アクション区画 }).childs([
                    button({ class: 切土盛土ボタン, text: '全ての道に合わせて地形を切土・盛土' })
                        .setTooltip('全ての道に合わせて地形を切土・盛土')
                        .onClick(() => this._配線.先.on道路切土盛土()),
                    this._操作ボタン群])])
        )
    }
}
