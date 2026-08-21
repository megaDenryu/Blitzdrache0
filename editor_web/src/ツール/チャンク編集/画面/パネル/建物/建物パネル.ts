import { div, span, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 建物外形定義 } from '../../../../../生成/編集資源契約.ts'
import {
    パネル,
    見出し行,
    生成ボタングリッド,
    生成ボタン,
    アクション区画,
    平坦化ボタン,
    行ボタン群,
    接地ボタン,
} from './スタイル.css.ts'
import { 建物件数ラベル } from './建物パネル/建物件数ラベル.ts'
import { 建物削除ボタン } from './建物パネル/建物削除ボタン.ts'

export interface I建物パネル配線 {
    readonly on建物生成: (建物定義ID: string) => void
    readonly on基礎平坦化: () => void
    readonly on地面接地: () => void
    readonly on建物削除: () => void
}

// 建物の新規配置・基礎平坦化・接地・削除操作を提供するLV2素部品。
export class 建物パネル extends LV2HtmlComponentBase implements I配線可能<I建物パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I建物パネル配線> = new 配線ポート<I建物パネル配線>('建物パネル')
    private readonly _件数表示: 建物件数ラベル
    private readonly _削除ボタン: 建物削除ボタン
    private readonly _生成ボタン領域: DivC = div({ class: 生成ボタングリッド })
    private readonly _診断表示 = span({ text: '' })
    private _カタログ診断 = ''

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

    public 建物定義一覧を更新する(定義一覧: ReadonlyArray<建物外形定義>): void {
        this._生成ボタン領域.clearChildren()
        this._カタログ診断 =
            定義一覧.length === 0 ? '配置できる建物定義がありません(外部アセットの置き場が無い環境では定義0件になります)' : ''
        this._診断表示.setTextContent(this._カタログ診断)
        for (const 定義 of 定義一覧.filter((候補) => 候補.用途 === '家屋')) {
            const 表示 = `+ ${定義.表示名}`
            this._生成ボタン領域.child(
                button({ class: 生成ボタン, text: 表示 })
                    .setTooltip(`${定義.表示名}（${定義.ベイ.横}×${定義.ベイ.奥}ベイ・${定義.ベイ.階}階）`)
                    .onClick(() => this._配線.先.on建物生成(定義.識別子)),
            )
        }
    }

    public カタログ取得失敗を表示する(説明: string): void {
        this._カタログ診断 = `建物外形カタログを取得できません: ${説明}`
        this._診断表示.setTextContent(this._カタログ診断)
    }

    public 未解決の建物定義を表示する(ID一覧: readonly string[]): void {
        const 未解決 = ID一覧.map((ID) => `定義未解決: ${ID}`).join(' / ')
        this._診断表示.setTextContent([this._カタログ診断, 未解決].filter((文言) => 文言.length > 0).join(' / '))
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し行 }).childs([
                    span({ text: '建物・目印の配置' }).setTooltip('建物・目印の配置'),
                    this._件数表示]),
                this._生成ボタン領域,
                this._診断表示,
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
