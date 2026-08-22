import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import { 使い方の閲覧記録を書く } from '../../境界/index.ts'
import {
    AIからの操作の節を組み立てる,
    この道具は何かの節を組み立てる,
    ゲームとして歩いて確かめるの節を組み立てる,
    テーマの切替の節を組み立てる,
    三次元ビューの操作の節を組み立てる,
    基本の流れの節を組み立てる,
    道の作り方と直し方の節を組み立てる,
} from './使い方の節.ts'
import { コンテナ, 本文幅, 表題 } from './スタイル.css.ts'

// エディタエリアに開く使い方ガイドのタブ内容。三次元ビューを持たない静的ページであり、
// 実行可能ツールの契約(寸法を合わせる・前面/背面になった)は空実装で満たす。
// 読み物の本文は使い方の節.tsが持ち、ここは節の並び順とタブとしての振る舞いだけを持つ。
export class 使い方タブ extends LV2HtmlComponentBase {
    protected _componentRoot: DivC

    public constructor() {
        super()
        this._componentRoot = div({ class: コンテナ }).child(
            div({ class: 本文幅 }).childs([
                div({ class: 表題, text: 'このエディターの使い方' }),
                この道具は何かの節を組み立てる(),
                基本の流れの節を組み立てる(),
                三次元ビューの操作の節を組み立てる(),
                道の作り方と直し方の節を組み立てる(),
                ゲームとして歩いて確かめるの節を組み立てる(),
                AIからの操作の節を組み立てる(),
                テーマの切替の節を組み立てる(),
            ]),
        )
    }

    public 寸法を合わせる(): void {}

    // 実際に前面へ出て見られたときに閲覧済みとして記録する(構築しただけでは記録しない)。
    public 前面になった(): void {
        使い方の閲覧記録を書く()
    }

    public 背面になった(): void {}
}
