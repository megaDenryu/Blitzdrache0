import type { HtmlComponentBase } from 'sengen-ui'

export interface 実行可能ツール extends HtmlComponentBase {
    寸法を合わせる(幅: number, 高さ: number): void
}

// 外殻レイアウトのアクティビティバーおよびエディタタブへ登録するツールの仕様。
export interface ツール項目 {
    readonly 識別子: string
    readonly ラベル: string
    readonly アイコン記号: string
    readonly ツールを生成する: () => 実行可能ツール
}
