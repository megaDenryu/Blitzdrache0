import { div, span, DivC } from 'sengen-ui'
import type { 楽曲ID } from '../../../境界/index.ts'
import { 木項目, 子木項目コンテナ, 子木項目, アイコン, フォルダアイコン } from '../スタイル.css.ts'

// エクスプローラーの「楽曲を作る」親ノードと、配下に並ぶ保存済みの楽曲の一覧。
export class 楽曲木 {
    private readonly _親ノード: DivC
    private readonly _子項目コンテナ: DivC = div({ class: 子木項目コンテナ })
    private readonly _ノードマップ: Map<楽曲ID, DivC> = new Map<楽曲ID, DivC>()

    public constructor(
        private readonly _on楽曲を開く: (楽曲ID: 楽曲ID, 表示名: string) => void,
        on楽曲を作る: () => void,
    ) {
        this._親ノード = div({ class: 木項目 })
            .childs([span({ class: フォルダアイコン, text: '＋' }), span({ text: '楽曲を作る' }).setTooltip('楽曲を作る')])
            .onClick(() => on楽曲を作る())
    }

    public get ルート要素一覧(): DivC[] {
        return [this._親ノード, this._子項目コンテナ]
    }

    public 一覧を作り直す(一覧: readonly 楽曲ID[]): void {
        this._子項目コンテナ.clearChildren()
        this._ノードマップ.clear()
        for (const 楽曲ID of 一覧) {
            const ノード = div({ class: 子木項目 })
                .childs([span({ class: アイコン, text: 'S' }), span({ text: 楽曲ID }).setTooltip(楽曲ID)])
                .onClick(() => this._on楽曲を開く(楽曲ID, 楽曲ID))
            this._ノードマップ.set(楽曲ID, ノード)
            this._子項目コンテナ.child(ノード)
        }
    }

    public 選択表示する(楽曲ID: 楽曲ID): void {
        for (const [識別子, ノード] of this._ノードマップ.entries()) {
            ノード.setAttribute('data-selected', String(識別子 === 楽曲ID))
        }
    }

    public 選択解除する(): void {
        for (const ノード of this._ノードマップ.values()) {
            ノード.setAttribute('data-selected', 'false')
        }
    }
}

