import { div, ButtonC, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { スライダー項目 } from '../../../../ワールド/画面/パネル/共通/スライダー項目.ts'
import { パネル, 見出し, アクション区画, 行ボタン群, 削除ボタン, 副ボタン } from './スタイル.css.ts'

export interface I広域道路パネル配線 {
    readonly on全幅変更: (全幅: number) => void
    readonly on細分割数変更: (細分割: number) => void
    readonly on選択ノード削除: () => void
    readonly on道路リセット: () => void
}

class 削除操作ボタン extends ButtonC {
    public constructor() {
        super({ class: 削除ボタン, text: '選択点を削除' })
        this.setAttribute('disabled', 'true')
    }

    public 有効状態を設定する(有効: boolean): this {
        if (有効) {
            this.dom.element.removeAttribute('disabled')
        } else {
            this.setAttribute('disabled', 'true')
        }
        return this
    }
}

// 広域幹線道路の幅・細分割数・ノード削除・全消去を管理するLV2素部品。
export class 広域道路パネル extends LV2HtmlComponentBase implements I配線可能<I広域道路パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I広域道路パネル配線> = new 配線ポート<I広域道路パネル配線>('広域道路パネル')
    private readonly _全幅スライダー: スライダー項目
    private readonly _細分割スライダー: スライダー項目
    private readonly _削除ボタン: 削除操作ボタン

    public constructor(初期全幅: number = 12.0, 初期細分割: number = 120) {
        super()
        this._全幅スライダー = new スライダー項目('道路全幅', 4, 32, 1, 初期全幅, 'm')
        this._細分割スライダー = new スライダー項目('カーブ細分割数', 40, 400, 20, 初期細分割)
        this._削除ボタン = new 削除操作ボタン()
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I広域道路パネル配線): this {
        this._配線.配線する(配線)
        this._全幅スライダー.配線する({ on値変更: (v: number) => this._配線.先.on全幅変更(v) })
        this._細分割スライダー.配線する({ on値変更: (v: number) => this._配線.先.on細分割数変更(v) })
        return this
    }

    public 選択ノード有効状態を設定する(有効: boolean): void {
        this._削除ボタン.有効状態を設定する(有効)
    }

    public override delete(): void {
        this._全幅スライダー.delete()
        this._細分割スライダー.delete()
        this._削除ボタン.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し, text: '広域幹線道路 (Cross-Chunk Highway)' }),
                this._全幅スライダー,
                this._細分割スライダー,
                div({ class: アクション区画 }).child(
                    div({ class: 行ボタン群 }).childs([
                        this._削除ボタン.onClick(() => this._配線.先.on選択ノード削除()),
                        new ButtonC({ class: 副ボタン, text: '全消去' }).onClick(() =>
                            this._配線.先.on道路リセット(),
                        ),
                    ]),
                ),
            ])
        )
    }
}
