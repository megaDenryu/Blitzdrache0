import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 楽曲 } from '../../../../生成/編集資源契約.ts'
import { コード進行パネル } from './コード進行/コード進行パネル.ts'
import { トラック設定パネル } from './トラック設定/トラック設定パネル.ts'
import { パターンパネル } from './パターン/パターンパネル.ts'
import { 曲構成パネル } from './曲構成/曲構成パネル.ts'
import { 曲設定パネル } from './曲設定/曲設定パネル.ts'

// 格子の下に並ぶ編集パネル5枚を、同じ1つの表示の更新で扱えるようにまとめて所有する。
// 楽曲編集画面はこの群を1つの子として置き、パネル1枚ずつの表示の同期を持たない。
export class 編集パネル群 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly パターン: パターンパネル
    public readonly 曲構成: 曲構成パネル
    public readonly 曲設定: 曲設定パネル
    public readonly トラック設定: トラック設定パネル
    public readonly コード進行: コード進行パネル

    public constructor(初期楽曲: 楽曲, 初期選択の名乗り: string | null) {
        super()
        this.パターン = new パターンパネル(初期楽曲, 初期選択の名乗り)
        this.曲構成 = new 曲構成パネル(初期楽曲, 初期選択の名乗り)
        this.曲設定 = new 曲設定パネル(初期楽曲)
        this.トラック設定 = new トラック設定パネル(初期楽曲)
        this.コード進行 = new コード進行パネル(初期楽曲)
        this._componentRoot = div().setStyleCSS({ display: 'flex', flexDirection: 'column', gap: '16px' }).childs([
            this.パターン,
            this.曲構成,
            this.曲設定,
            this.トラック設定,
            this.コード進行,
        ])
    }

    public 表示を更新する(楽曲: 楽曲, 選択中パターンの名乗り: string | null): void {
        this.パターン.表示を更新する(楽曲, 選択中パターンの名乗り)
        this.曲構成.表示を更新する(楽曲, 選択中パターンの名乗り)
        this.曲設定.表示を更新する(楽曲)
        this.トラック設定.表示を更新する(楽曲)
        this.コード進行.表示を更新する(楽曲)
    }

    public override delete(): void {
        this.パターン.delete()
        this.曲構成.delete()
        this.曲設定.delete()
        this.トラック設定.delete()
        this.コード進行.delete()
        super.delete()
    }
}
