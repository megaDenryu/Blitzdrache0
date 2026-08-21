import { div, button, DivC, ButtonC, LV2HtmlComponentBase } from 'sengen-ui'
import type { マテリアル台帳 } from '../../../生成/編集資源契約.ts'
import { 永続化パネル } from '../../チャンク編集/画面/パネル/永続化/index.ts'
import { マテリアル一覧部品, type Iマテリアル一覧配線 } from './マテリアル一覧部品.ts'
import { 層割当パネル, type I層割当パネル配線 } from './層割当パネル.ts'
import { コンテナ, 本文幅, 表題, セクション, セクション見出し, 追加ボタン } from './スタイル.css.ts'

// マテリアル台帳タブの画面全体。マテリアル一覧・層割当・保存永続化を1枚に並べる文書タブであり、
// 三次元ビューを持たない(判断9: 登録と識別色の定義までがこのエディターの責務)。
export class マテリアル台帳パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 一覧: マテリアル一覧部品 = new マテリアル一覧部品()
    public readonly 層割当: 層割当パネル = new 層割当パネル()
    public readonly 永続化: 永続化パネル = new 永続化パネル()
    private readonly _追加ボタン: ButtonC

    public constructor() {
        super()
        this._追加ボタン = button({ class: 追加ボタン, text: '+ マテリアルを追加' }).setTooltip('マテリアルを追加')
        this._componentRoot = div({ class: コンテナ }).child(
            div({ class: 本文幅 }).childs([
                div({ class: 表題, text: 'マテリアル' }),
                div({ class: セクション }).childs([
                    div({ class: セクション見出し, text: '登録済みマテリアル' }),
                    this.一覧,
                    this._追加ボタン,
                ]),
                div({ class: セクション }).childs([
                    div({ class: セクション見出し, text: '地表材質の割当' }),
                    this.層割当,
                ]),
                this.永続化,
            ]),
        )
    }

    // タブ構築時に一度だけ呼ぶ。層割当のセレクトと追加ボタンのイベント購読はここに閉じ、
    // 表示を更新するでは再購読しない(選択肢の再構築のたびに購読が積み増るのを避けるため)。
    public 配線する(層割当配線: I層割当パネル配線, on材質追加: () => void): void {
        this.層割当.配線する(層割当配線)
        this._追加ボタン.onClick(() => on材質追加())
    }

    // 一覧の行数が変わる操作(読込・追加・削除)のあとに呼ぶ。行を丸ごと作り直す。
    public 表示を更新する(台帳: マテリアル台帳, 一覧配線: Iマテリアル一覧配線): void {
        this.一覧.再構築する(台帳.マテリアル一覧, 一覧配線)
        this.層割当の選択肢だけを更新する(台帳)
    }

    // 材質名の変更のあとに呼ぶ。行の入力欄はフォーカスを保つため再構築せず、
    // 層割当セレクトの選択肢テキストだけを追従させる。
    public 層割当の選択肢だけを更新する(台帳: マテリアル台帳): void {
        const 材質名一覧 = 台帳.マテリアル一覧.map((定義) => 定義.エンジン材質名)
        this.層割当.選択肢を更新する(材質名一覧, 台帳.層割当)
    }

    public override delete(): void {
        this.一覧.delete()
        this.層割当.delete()
        this.永続化.delete()
        this._追加ボタン.delete()
        super.delete()
    }
}
