import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 実行可能ツール } from './ツール定義.ts'
import { タブ管理サービス } from './タブ管理サービス.ts'

class 偽ツール implements 実行可能ツール {
    public 前面化回数 = 0
    public 背面化回数 = 0
    public 破棄回数 = 0
    public 寸法合わせ回数 = 0

    public 寸法を合わせる(): void {
        this.寸法合わせ回数++
    }
    public 前面になった(): void {
        this.前面化回数++
    }
    public 背面になった(): void {
        this.背面化回数++
    }
    public delete(): void {
        this.破棄回数++
    }
}

describe('タブ管理サービスのライフサイクルテスト', () => {
    it('タブ選択で直前のツールが背面化し新ツールが前面化すること', () => {
        const 管理 = new タブ管理サービス()
        const ツール1 = new 偽ツール()
        const ツール2 = new 偽ツール()

        管理.ツールを登録する('tab1', ツール1)
        管理.ツールを登録する('tab2', ツール2)

        管理.タブを選択する('tab1')
        assert.strictEqual(ツール1.前面化回数, 1)
        assert.strictEqual(ツール1.背面化回数, 0)
        assert.strictEqual(ツール2.前面化回数, 0)

        管理.タブを選択する('tab2')
        assert.strictEqual(ツール1.背面化回数, 1)
        assert.strictEqual(ツール2.前面化回数, 1)
    })

    it('タブ破棄時にツールが背面化されて破棄されること', () => {
        const 管理 = new タブ管理サービス()
        const ツール = new 偽ツール()

        管理.ツールを登録する('tab1', ツール)
        管理.タブを選択する('tab1')
        管理.タブを破棄する('tab1')

        assert.strictEqual(ツール.背面化回数, 1)
        assert.strictEqual(ツール.破棄回数, 1)
        assert.strictEqual(管理.前面ツールを取得する(), undefined)
    })
})
