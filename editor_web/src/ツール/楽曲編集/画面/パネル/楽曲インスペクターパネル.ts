import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 楽曲 } from '../../../../生成/編集資源契約.ts'
import { 永続化パネル } from '../../../チャンク編集/画面/パネル/永続化/index.ts'
import { 編集パネル群 } from './編集パネル群.ts'
import { 楽曲インスペクター枠 } from './スタイル.css.ts'

// 右サイドバーへ出す楽曲の設定一式。パターン・曲構成・ミキサー・トラック設定・コード進行と、保存と読み込みを収める。
// 中央には編集の対象そのもの(打ち込みの格子)だけを残すため、設定はここへ集める(設計正本の判断14)。
export class 楽曲インスペクターパネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly パネル群: 編集パネル群
    public readonly 永続化: 永続化パネル = new 永続化パネル()

    public constructor(初期楽曲: 楽曲, 初期選択の名乗り: string | null) {
        super()
        this.パネル群 = new 編集パネル群(初期楽曲, 初期選択の名乗り)
        this._componentRoot = div({ class: 楽曲インスペクター枠 }).childs([this.パネル群, this.永続化])
    }

    public 表示を更新する(楽曲: 楽曲, 選択中パターンの名乗り: string | null): void {
        this.パネル群.表示を更新する(楽曲, 選択中パターンの名乗り)
    }

    public override delete(): void {
        this.パネル群.delete()
        this.永続化.delete()
        super.delete()
    }
}
