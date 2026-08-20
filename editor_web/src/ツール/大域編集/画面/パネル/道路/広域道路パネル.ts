import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 道路の設定 } from '../../../../チャンク編集/編集モデル/index.ts'
import { スライダー項目 } from '../../../../チャンク編集/画面/パネル/共通/スライダー項目.ts'
import {
    道路の操作ボタン群,
    type I道路の操作ボタン群配線,
} from '../../../../チャンク編集/画面/パネル/共通/道路の操作ボタン群.ts'
import { パネル, 見出し, アクション区画 } from './スタイル.css.ts'

export interface I広域道路パネル配線 extends I道路の操作ボタン群配線 {
    readonly on全幅変更: (全幅: number) => void
    readonly on細分割数変更: (細分割: number) => void
}

// 広域幹線道路の幅・細分割数の設定と、道の追加・削除を管理するLV2素部品。
// 設定は「対象の道」1本にだけ効く(幹線は分岐して何本でも置ける)。
export class 広域道路パネル extends LV2HtmlComponentBase implements I配線可能<I広域道路パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I広域道路パネル配線> = new 配線ポート<I広域道路パネル配線>('広域道路パネル')
    private readonly _全幅スライダー: スライダー項目
    private readonly _細分割スライダー: スライダー項目
    private readonly _操作ボタン群: 道路の操作ボタン群

    public constructor(初期全幅: number = 12.0, 初期細分割: number = 120) {
        super()
        this._全幅スライダー = new スライダー項目('道路全幅', 4, 32, 1, 初期全幅, 'm')
        this._細分割スライダー = new スライダー項目('カーブ細分割数', 40, 400, 20, 初期細分割)
        this._操作ボタン群 = new 道路の操作ボタン群()
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I広域道路パネル配線): this {
        this._配線.配線する(配線)
        this._全幅スライダー.配線する({ on値変更: (v: number) => this._配線.先.on全幅変更(v) })
        this._細分割スライダー.配線する({ on値変更: (v: number) => this._配線.先.on細分割数変更(v) })
        this._操作ボタン群.配線する(配線)
        return this
    }

    public 選択中の道路点があるか設定する(選択あり: boolean): void {
        this._操作ボタン群.選択中の道路点があるか設定する(選択あり)
    }

    // 設定が効く道を切り替える。設定がnullのときは対象の道が無く、スライダーを操作させない。
    public 対象の道を設定する(道路添字: number | null, 本数: number, 設定: 道路の設定 | null): void {
        this._操作ボタン群.対象の道を設定する(道路添字, 本数)
        this._全幅スライダー.操作できるか設定する(設定 !== null)
        this._細分割スライダー.操作できるか設定する(設定 !== null)
        if (設定 === null) return
        this._全幅スライダー.値を設定する(設定.全幅メートル)
        this._細分割スライダー.値を設定する(設定.細分割数)
    }

    public override delete(): void {
        this._全幅スライダー.delete()
        this._細分割スライダー.delete()
        this._操作ボタン群.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し, text: '広域幹線道路' }).setTooltip('広域幹線道路'),
                this._全幅スライダー,
                this._細分割スライダー,
                div({ class: アクション区画 }).child(this._操作ボタン群)])
        )
    }
}
