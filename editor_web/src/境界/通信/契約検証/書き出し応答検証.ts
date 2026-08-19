import { オブジェクトか, 数値か, 文字列か } from './オブジェクト判定.ts'

export interface 書き出し応答本体 {
    readonly 書いたファイル数: number
    readonly 出力先: string
}

// `POST /api/書き出し/ソースアセット`の成功応答(200)が型契約に適合しているかを実行時に検査する。
export function 書き出し応答の形か(値: unknown): 値 is 書き出し応答本体 {
    if (!オブジェクトか(値)) return false
    return 数値か(値['書いたファイル数']) && 文字列か(値['出力先'])
}
