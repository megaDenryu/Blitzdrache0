import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 進行状況表示パネル } from '../../../永続化パネル操作.ts'
import { 永続化の操作列 } from './永続化の操作列.ts'
import { 永続化枠 } from './スタイル.css.ts'

// 保存・読み込みの操作ボタンと現在の通信結果・エラー文言を表示するパネル。
// 振る舞いは永続化の操作列が持ち、ここは右サイドバーの枠の見た目だけを持って委譲する。
export class 永続化パネル extends LV2HtmlComponentBase implements 進行状況表示パネル {
    protected _componentRoot: DivC
    private readonly _操作列: 永続化の操作列 = new 永続化の操作列()

    public constructor() {
        super()
        this._componentRoot = div({ class: 永続化枠 }).child(this._操作列)
    }

    public on保存クリック(コールバック: () => void): void {
        this._操作列.on保存クリック(コールバック)
    }

    public on読込クリック(コールバック: () => void): void {
        this._操作列.on読込クリック(コールバック)
    }

    public ボタン活性状態を更新する(活性: boolean): void {
        this._操作列.ボタン活性状態を更新する(活性)
    }

    public 状態文言を更新する(文言: string, エラーか: boolean = false): void {
        this._操作列.状態文言を更新する(文言, エラーか)
    }

    public override delete(): void {
        this._操作列.delete()
        super.delete()
    }
}
