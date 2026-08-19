import type { HtmlComponentBase } from 'sengen-ui'

// エディタタブに配置され寸法調整・インスペクター提供・前面背面ライフサイクルを持つツールの仕様。
export interface 実行可能ツール {
    寸法を合わせる(幅: number, 高さ: number): void
    前面になった(): void
    背面になった(): void
    delete(): void
    readonly インスペクター?: HtmlComponentBase
}
