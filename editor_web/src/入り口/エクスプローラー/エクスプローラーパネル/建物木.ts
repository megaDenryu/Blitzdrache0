import { div, span, DivC } from 'sengen-ui'
import type { 建物の格子の一覧項目 } from '../../../境界/通信/index.ts'
import { 木項目, 子木項目コンテナ, 子木項目, アイコン, フォルダアイコン } from '../スタイル.css.ts'

// エクスプローラーの「建物」親ノードと、その配下に並ぶ保存済みの建物の一覧。
// 一覧はサーバーが持つ台帳が正本であるため、この型は渡された並びを描くだけで自分では持たない。
export class 建物木 {
    private readonly _親ノード: DivC
    private readonly _子項目コンテナ: DivC = div({ class: 子木項目コンテナ })
    private readonly _ノードマップ: Map<string, DivC> = new Map<string, DivC>()

    public constructor(
        private readonly _on建物を開く: (建物定義ID: string, 表示名: string) => void,
        on建物を作る: () => void,
    ) {
        this._親ノード = div({ class: 木項目 })
            .childs([span({ class: フォルダアイコン, text: '＋' }), span({ text: '建物を作る' }).setTooltip('建物を作る')])
            .onClick(() => on建物を作る())
    }

    public get ルート要素一覧(): DivC[] {
        return [this._親ノード, this._子項目コンテナ]
    }

    public 一覧を作り直す(一覧: readonly 建物の格子の一覧項目[]): void {
        this._子項目コンテナ.clearChildren()
        this._ノードマップ.clear()
        for (const 項目 of 一覧) {
            const ラベル = `${項目.表示名} (${項目.升目の数}升目)`
            const ノード = div({ class: 子木項目 })
                .childs([span({ class: アイコン, text: 'B' }), span({ text: ラベル }).setTooltip(項目.識別子)])
                .onClick(() => this._on建物を開く(項目.識別子, 項目.表示名))
            this._ノードマップ.set(項目.識別子, ノード)
            this._子項目コンテナ.child(ノード)
        }
    }

    public 選択表示する(建物定義ID: string): void {
        for (const [識別子, ノード] of this._ノードマップ.entries()) {
            ノード.setAttribute('data-selected', String(識別子 === 建物定義ID))
        }
    }

    public 選択解除する(): void {
        for (const ノード of this._ノードマップ.values()) {
            ノード.setAttribute('data-selected', 'false')
        }
    }
}
