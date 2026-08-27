// セレクトへ渡す選択肢1件の形。パネルの各所で組み立てる選択肢一覧が共通で話す語彙である。
export type セレクトの選択肢 = {
    readonly value: string
    readonly text: string
    readonly selected: boolean
}
