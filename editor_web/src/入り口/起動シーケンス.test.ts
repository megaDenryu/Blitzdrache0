import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 実行可能ツール } from './ツール定義.ts'
import { タブ管理サービス } from './タブ管理サービス.ts'
import { 起動時タブ計画を立てる } from './起動時タブ計画.ts'

class 偽ツール implements 実行可能ツール {
    public 前面化回数 = 0
    public 寸法を合わせる(): void {}
    public 前面になった(): void {
        this.前面化回数++
    }
    public 背面になった(): void {}
    public delete(): void {}
}

interface 再現結果 {
    readonly タブ管理: タブ管理サービス
    readonly 使い方タブ: 偽ツール | null
}

// エディター外殻の起動シーケンス(起動時タブ計画を立てる→計画順にタブ管理へ登録して選択する)を、
// 実DOMを要さないタブ管理サービスとダミーツールで再現する。sengen-uiの実部品(使い方タブ)は
// 実DOMを要するためNodeのテスト環境では構築できず、代わりにタブ管理への積み方だけを検査する。
function 起動シーケンスを再現する(使い方ガイド閲覧済みか: boolean): 再現結果 {
    const タブ管理 = new タブ管理サービス()
    let 使い方タブ: 偽ツール | null = null
    for (const 種別 of 起動時タブ計画を立てる(使い方ガイド閲覧済みか)) {
        if (種別 === '大域世界') {
            const ツール = new 偽ツール()
            タブ管理.ツールを登録する('大域世界', ツール)
            タブ管理.タブを選択する('大域世界')
        } else {
            使い方タブ = new 偽ツール()
            タブ管理.ツールを登録する('使い方', 使い方タブ)
            タブ管理.タブを選択する('使い方')
        }
    }
    return { タブ管理, 使い方タブ }
}

describe('エディター外殻の起動シーケンス', () => {
    it('初回起動(使い方ガイド未閲覧)では使い方タブがタブ管理へ積まれ前面になること', () => {
        const { タブ管理, 使い方タブ } = 起動シーケンスを再現する(false)

        assert.notStrictEqual(使い方タブ, null)
        assert.strictEqual(タブ管理.前面ツールを取得する(), 使い方タブ)
        assert.strictEqual(使い方タブ?.前面化回数, 1)
    })

    it('2回目以降(使い方ガイド閲覧済み)では使い方タブがタブ管理へ積まれないこと', () => {
        const { タブ管理, 使い方タブ } = 起動シーケンスを再現する(true)

        assert.strictEqual(使い方タブ, null)
        assert.notStrictEqual(タブ管理.前面ツールを取得する(), null)
    })
})
