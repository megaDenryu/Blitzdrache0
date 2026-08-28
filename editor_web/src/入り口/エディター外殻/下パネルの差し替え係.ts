import type { HtmlComponentBase } from 'sengen-ui'

// 下パネルの開閉を握る相手。外殻レイアウトが満たす。
export interface I下パネルを開閉できる外殻 {
    パネル利用可能を設定する(利用可能: boolean): void
}

// 下パネルの中身を差し替える置き場。DivCが満たす。
export interface I下パネルのスロット {
    clearChildren(): void
    child(部品: HtmlComponentBase): void
}

// 前面のタブが変わるたび、そのツールが持つ下パネルの部品をスロットへ入れ替え、
// 下パネルを持たないツールでは下パネルそのものを閉じる操作サービス。
//
// 閉じるところまでをこの型が受け持つのは、中身を空にするだけでは空のパネルが帯として画面に残り、
// 「何も無い領域が常設されている」という以前の不具合が再発するためである。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class 下パネルの差し替え係 {
    public constructor(
        private readonly _シェル: I下パネルを開閉できる外殻,
        private readonly _スロット: I下パネルのスロット,
    ) {}

    public 前面のツールに合わせる(下パネル: HtmlComponentBase | undefined): void {
        this._スロット.clearChildren()
        if (下パネル !== undefined) this._スロット.child(下パネル)
        this._シェル.パネル利用可能を設定する(下パネル !== undefined)
    }

    public 空にする(): void {
        this.前面のツールに合わせる(undefined)
    }
}
