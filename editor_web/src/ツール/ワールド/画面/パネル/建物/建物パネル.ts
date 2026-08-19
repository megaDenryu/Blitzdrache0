import { div, span, button, DivC, SpanC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 建物種別 } from '../../../../../生成/編集資源契約.ts'
import {
    パネル,
    見出し行,
    件数ラベル,
    生成ボタングリッド,
    生成ボタン,
    アクション区画,
    平坦化ボタン,
    行ボタン群,
    接地ボタン,
    削除ボタン,
} from './スタイル.css.ts'

export interface I建物パネル配線 {
    readonly on建物生成: (種別: 建物種別) => void
    readonly on基礎平坦化: () => void
    readonly on地面接地: () => void
    readonly on建物削除: () => void
}

class 建物件数ラベル extends SpanC {
    public constructor(初期件数: number) {
        super({ class: 件数ラベル, text: `${初期件数} 件` })
    }

    public 件数を更新する(件数: number): this {
        this.setTextContent(`${件数} 件`)
        return this
    }
}

class 建物削除ボタン extends ButtonC {
    public constructor() {
        super({ class: 削除ボタン, text: '削除', disabled: true })
        this.setTooltip('削除')
    }

    public 有効状態を設定する(有効: boolean): this {
        this.setDisabled(!有効)
        return this
    }
}

// 建物の新規配置・基礎平坦化・接地・削除操作を提供するLV2素部品。
export class 建物パネル extends LV2HtmlComponentBase implements I配線可能<I建物パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I建物パネル配線> = new 配線ポート<I建物パネル配線>('建物パネル')
    private readonly _件数表示: 建物件数ラベル
    private readonly _削除ボタン: 建物削除ボタン

    public constructor() {
        super()
        this._件数表示 = new 建物件数ラベル(0)
        this._削除ボタン = new 建物削除ボタン()
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I建物パネル配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 件数を更新する(件数: number): void {
        this._件数表示.件数を更新する(件数)
    }

    public 選択建物有効状態を設定する(選択あり: boolean): void {
        this._削除ボタン.有効状態を設定する(選択あり)
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し行 }).childs([
                    span({ text: '建物・目印の配置' }).setTooltip('建物・目印の配置'),
                    this._件数表示]),
                div({ class: 生成ボタングリッド }).childs([
                    button({ class: 生成ボタン, text: '+ 家屋 (12m)' })
                        .setTooltip('+ 家屋 (12m)')
                        .onClick(() => this._配線.先.on建物生成('家屋')),
                    button({ class: 生成ボタン, text: '+ 塔 (8m)' })
                        .setTooltip('+ 塔 (8m)')
                        .onClick(() => this._配線.先.on建物生成('塔')),
                    button({ class: 生成ボタン, text: '+ 宝箱 (2m)' })
                        .setTooltip('+ 宝箱 (2m)')
                        .onClick(() => this._配線.先.on建物生成('宝箱'))]),
                div({ class: アクション区画 }).childs([
                    button({ class: 平坦化ボタン, text: '選択建物の基礎に合わせて地形造成' })
                        .setTooltip('選択建物の基礎に合わせて地形造成')
                        .onClick(() => this._配線.先.on基礎平坦化()),
                    div({ class: 行ボタン群 }).childs([
                        button({ class: 接地ボタン, text: '地面に接地' })
                            .setTooltip('地面に接地')
                            .onClick(() => this._配線.先.on地面接地()),
                        this._削除ボタン
                            .onClick(() => this._配線.先.on建物削除())])])])
        )
    }
}
