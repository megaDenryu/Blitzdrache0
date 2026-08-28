import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { HtmlComponentBase } from 'sengen-ui'
import { 下パネルの差し替え係, type I下パネルのスロット, type I下パネルを開閉できる外殻 } from './下パネルの差し替え係.ts'

// DOMを持たない場所で検証するため、スロットと外殻の記録役を立てる。
class 記録する外殻 implements I下パネルを開閉できる外殻 {
    public 最後に伝えた利用可能: boolean | undefined = undefined

    public パネル利用可能を設定する(利用可能: boolean): void {
        this.最後に伝えた利用可能 = 利用可能
    }
}

class 記録するスロット implements I下パネルのスロット {
    public 入っている部品: HtmlComponentBase[] = []

    public clearChildren(): void {
        this.入っている部品 = []
    }

    public child(部品: HtmlComponentBase): void {
        this.入っている部品.push(部品)
    }
}

// 部品の中身は差し替え係にとって関係が無いため、同一性だけ見分けられる印を渡す。
function 部品の代わりの印(): HtmlComponentBase {
    return {} as unknown as HtmlComponentBase
}

describe('下パネルの差し替え係', () => {
    it('下パネルを持つツールでは、その部品をスロットへ入れて下パネルを利用可能にする', () => {
        const 外殻 = new 記録する外殻()
        const スロット = new 記録するスロット()
        const 差し替え = new 下パネルの差し替え係(外殻, スロット)
        const 棚 = 部品の代わりの印()

        差し替え.前面のツールに合わせる(棚)

        assert.deepStrictEqual(スロット.入っている部品, [棚])
        assert.strictEqual(外殻.最後に伝えた利用可能, true)
    })

    it('下パネルを持たないツールでは、スロットを空にして下パネルを利用不可にする', () => {
        const 外殻 = new 記録する外殻()
        const スロット = new 記録するスロット()
        const 差し替え = new 下パネルの差し替え係(外殻, スロット)

        差し替え.前面のツールに合わせる(undefined)

        assert.deepStrictEqual(スロット.入っている部品, [])
        assert.strictEqual(外殻.最後に伝えた利用可能, false)
    })

    it('下パネルを持つツールから持たないツールへ移ると、前の棚が残らず利用不可へ戻る', () => {
        const 外殻 = new 記録する外殻()
        const スロット = new 記録するスロット()
        const 差し替え = new 下パネルの差し替え係(外殻, スロット)

        差し替え.前面のツールに合わせる(部品の代わりの印())
        差し替え.前面のツールに合わせる(undefined)

        assert.deepStrictEqual(スロット.入っている部品, [])
        assert.strictEqual(外殻.最後に伝えた利用可能, false)
    })

    it('タブが1つも無くなったときは空にするで下パネルを閉じる', () => {
        const 外殻 = new 記録する外殻()
        const スロット = new 記録するスロット()
        const 差し替え = new 下パネルの差し替え係(外殻, スロット)

        差し替え.前面のツールに合わせる(部品の代わりの印())
        差し替え.空にする()

        assert.deepStrictEqual(スロット.入っている部品, [])
        assert.strictEqual(外殻.最後に伝えた利用可能, false)
    })
})
