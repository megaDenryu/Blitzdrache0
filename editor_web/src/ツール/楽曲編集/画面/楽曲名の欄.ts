import { div, textInput, DivC, TextInputC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲 } from '../../../生成/編集資源契約.ts'
import { 名乗りの添え, 楽曲名の入力, 楽曲名の枠 } from './スタイル.css.ts'

// 入力の途中と入力の確定を別の口にする。途中はコマンドを積まず見えだけを追随させ、
// 確定したときに取り消し1回ぶんのコマンドを積むためである(設計正本の判断13)。
export interface I楽曲名の欄配線 {
    readonly on表示名が入力された: (入力中の表示名: string) => void
    readonly on表示名が決まった: (新しい表示名: string) => void
}

// いま編集している楽曲の表示名を出し、その場で書き換える欄。
// 演奏の操作帯と同じ行へ置いて名前のためだけの行を無くすため、格子の上の固定の行が持つ(設計正本の判断14)。
// 同じ値を2箇所で変えられる形にしないため、右サイドバーの設定側にはこの欄を置かない。
export class 楽曲名の欄 extends LV2HtmlComponentBase implements I配線可能<I楽曲名の欄配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I楽曲名の欄配線> = new 配線ポート<I楽曲名の欄配線>('楽曲名の欄')
    private readonly _表示名入力: TextInputC
    private readonly _名乗り表示: DivC
    private _打っている最中か = false

    public constructor(初期楽曲: 楽曲) {
        super()
        this._表示名入力 = textInput({ class: 楽曲名の入力, value: 初期楽曲.表示名, placeholder: '楽曲の表示名' })
            .setTooltip('楽曲の表示名。ここで変えると文書タブの見出しも変わる')
            .addTypedEventListener('focus', () => { this._打っている最中か = true })
            .addTypedEventListener('blur', () => { this._打っている最中か = false })
        this._名乗り表示 = div({ class: 名乗りの添え, text: 初期楽曲.名乗り }).setTooltip('保存先を決める名乗り。変えられない')
        this._componentRoot = div({ class: 楽曲名の枠 }).childs([this._表示名入力, this._名乗り表示])
    }

    public 配線する(配線: I楽曲名の欄配線): this {
        this._配線.配線する(配線)
        this._表示名入力.onInput(() => this._入力された())
        this._表示名入力.onChange(() => this._入力が決まった())
        return this
    }

    // 打っている最中は正本の値で上書きしない。打っている間はコマンドを積まないため正本が古く、
    // この間に演奏や打ち込みで画面が作り直されると、打った文字が正本の綴りへ戻ってしまう。
    // changeはblurより先に届くため、欄から離れるときは確定のコマンドが先に積まれる。
    public 表示を更新する(楽曲: 楽曲): void {
        if (this._打っている最中か) return
        if (this._表示名入力.getValue() !== 楽曲.表示名) this._表示名入力.setValue(楽曲.表示名)
        this._名乗り表示.setTextContent(楽曲.名乗り)
    }

    public override delete(): void {
        this._表示名入力.delete()
        this._名乗り表示.delete()
        super.delete()
    }

    // 打っている間。onInputは1文字ごとに来るため、ここでコマンドを積んではならない。
    private _入力された(): void {
        if (this._配線.配線済みか) this._配線.先.on表示名が入力された(this._表示名入力.getValue())
    }

    // 欄から離れたとき、または入力が確定したとき(changeはその両方で1回だけ来る)。
    private _入力が決まった(): void {
        if (this._配線.配線済みか) this._配線.先.on表示名が決まった(this._表示名入力.getValue())
    }
}
