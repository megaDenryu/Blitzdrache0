import type { HtmlComponentBase } from 'sengen-ui'

export interface ツール項目 {
    readonly 識別子: string
    readonly 表示名: string
    readonly 作成: () => HtmlComponentBase
}
