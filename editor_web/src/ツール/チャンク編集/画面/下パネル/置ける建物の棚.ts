import { div, button, DivC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 建物外形定義 } from '../../../../生成/編集資源契約.ts'
import { 建物定義IDを生成する, type 建物定義ID } from '../../../../境界/建物定義ID.ts'
import { 生成ボタングリッド, 生成ボタン } from '../パネル/建物/スタイル.css.ts'
import { 棚のカード, 棚のカードの見出し, 棚の案内文 } from '../パネル/共通/棚.css.ts'

export interface I置ける建物の棚配線 {
    readonly on建物を置く: (建物定義ID: 建物定義ID) => void
}

// 建物の定義が引けない事情を出す札(LV1拡張)。定義が0件のままだと1件も置けないため、
// 利用者が理由を知れないと道具が壊れたようにしか見えない。
class 建物の定義の断りの札 extends SpanC {
    public constructor() {
        super({ class: 棚の案内文 })
    }

    public 文言を設定する(文言: string): this {
        this.setTextContent(文言)
        this.setTooltip(文言)
        return this
    }
}

// 下パネルの棚へ置く、これからチャンクへ置ける建物の一覧。押すとその建物が置かれる。
// 置いた建物への操作(接地・削除)を持たないのは、それが「選んでいるものへの操作」であり
// 右サイドバーの持ち物だからである(設計正本の判断14)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class 置ける建物の棚 extends LV2HtmlComponentBase implements I配線可能<I置ける建物の棚配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I置ける建物の棚配線> = new 配線ポート<I置ける建物の棚配線>('置ける建物の棚')
    private readonly _建物の並び: DivC = div({ class: 生成ボタングリッド })
    private readonly _断りの札: 建物の定義の断りの札 = new 建物の定義の断りの札()
    private _カタログの断り = ''

    public constructor() {
        super()
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I置ける建物の棚配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 建物定義一覧を更新する(定義一覧: ReadonlyArray<建物外形定義>): void {
        this._建物の並び.clearChildren()
        this._カタログの断り = 定義一覧.length === 0
            ? '置ける建物の定義がありません(外部アセットの置き場が無い環境では定義0件になります)'
            : ''
        this._断りの札.文言を設定する(this._カタログの断り)
        for (const 定義 of 定義一覧.filter((候補) => 候補.用途 === '家屋')) {
            this._建物の並び.child(
                button({ class: 生成ボタン, text: 定義.表示名 })
                    .setTooltip(`${定義.表示名}（${定義.ベイ.横}×${定義.ベイ.奥}ベイ・${定義.ベイ.階}階）を置く`)
                    .onClick(() => this._配線.先.on建物を置く(建物定義IDを生成する(定義.識別子))),
            )
        }
    }

    public カタログ取得失敗を表示する(説明: string): void {
        this._カタログの断り = `置ける建物の一覧を取得できません: ${説明}`
        this._断りの札.文言を設定する(this._カタログの断り)
    }

    public 未解決の建物定義を表示する(ID一覧: readonly 建物定義ID[]): void {
        const 未解決 = ID一覧.map((ID) => `定義が引けない建物: ${ID}`).join(' / ')
        this._断りの札.文言を設定する([this._カタログの断り, 未解決].filter((文言) => 文言.length > 0).join(' / '))
    }

    public override delete(): void {
        this._断りの札.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: 棚のカード }).childs([
                div({ class: 棚のカードの見出し, text: '置ける建物' }),
                this._建物の並び,
                this._断りの札])
        )
    }
}
