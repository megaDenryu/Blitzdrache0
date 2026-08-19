import { div, span, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { スライダー項目 } from '../共通/スライダー項目.ts'
import { パネル, 見出し, アクション区画, 切土盛土ボタン, 行ボタン群, 削除ボタン, 副ボタン } from './スタイル.css.ts'

export interface I道路パネル配線 {
    readonly on全幅変更: (全幅: number) => void
    readonly on除外バッファ変更: (バッファ: number) => void
    readonly on細分割数変更: (細分割: number) => void
    readonly on道路切土盛土: () => void
    readonly on選択ノード削除: () => void
    readonly on道路リセット: () => void
}

class 道路削除ボタン extends ButtonC {
    public constructor() {
        super({ class: 削除ボタン, text: 'ノード削除', disabled: true })
        this.setTooltip('ノード削除')
    }

    public 有効状態を設定する(有効: boolean): this {
        this.setDisabled(!有効)
        return this
    }
}

// 道路の幅・散布除外バッファ・細分割数設定および切土盛土造成を提供するLV2素部品。
export class 道路パネル extends LV2HtmlComponentBase implements I配線可能<I道路パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I道路パネル配線> = new 配線ポート<I道路パネル配線>('道路パネル')
    private readonly _全幅スライダー: スライダー項目
    private readonly _除外バッファスライダー: スライダー項目
    private readonly _細分割スライダー: スライダー項目
    private readonly _削除ボタン: 道路削除ボタン

    public constructor(初期全幅: number, 初期バッファ: number, 初期細分割: number) {
        super()
        this._全幅スライダー = new スライダー項目('道路全幅', 2, 24, 0.5, 初期全幅, 'm')
        this._除外バッファスライダー = new スライダー項目('散布除外バッファ', 4, 40, 1, 初期バッファ, 'm')
        this._細分割スライダー = new スライダー項目('細分割数', 20, 300, 10, 初期細分割)
        this._削除ボタン = new 道路削除ボタン()

        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I道路パネル配線): this {
        this._配線.配線する(配線)
        this._全幅スライダー.配線する({ on値変更: (v) => this._配線.先.on全幅変更(v) })
        this._除外バッファスライダー.配線する({ on値変更: (v) => this._配線.先.on除外バッファ変更(v) })
        this._細分割スライダー.配線する({ on値変更: (v) => this._配線.先.on細分割数変更(v) })
        return this
    }

    public 選択ノード有効状態を設定する(選択あり: boolean): void {
        this._削除ボタン.有効状態を設定する(選択あり)
    }

    public override delete(): void {
        this._全幅スライダー.delete()
        this._除外バッファスライダー.delete()
        this._細分割スライダー.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                span({ class: 見出し, text: '道路 & パラメータ' }).setTooltip('道路 & パラメータ'),
                this._全幅スライダー,
                this._除外バッファスライダー,
                this._細分割スライダー,
                div({ class: アクション区画 }).childs([
                    button({ class: 切土盛土ボタン, text: '道路に合わせて地形を切土・盛土' })
                        .setTooltip('道路に合わせて地形を切土・盛土')
                        .onClick(() => this._配線.先.on道路切土盛土()),
                    div({ class: 行ボタン群 }).childs([
                        this._削除ボタン
                            .onClick(() => this._配線.先.on選択ノード削除()),
                        button({ class: 副ボタン, text: 'リセット' })
                            .setTooltip('リセット')
                            .onClick(() => this._配線.先.on道路リセット())])])])
        )
    }
}
