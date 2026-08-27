import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 楽曲編集キーボード入力を配線する } from './キーボード配線.ts'
import { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'

describe('楽曲編集キーボード配線', () => {
    it('Ctrl+Zで取り消しが呼ばれAltで進行の外モードがトグルすること', () => {
        const UI状態 = new 楽曲編集UI状態()
        let 取り消し回数 = 0
        let 同期回数 = 0

        const 操作 = {
            直前の操作を取り消す: () => {
                取り消し回数++
                return true
            },
        } as unknown as Parameters<typeof 楽曲編集キーボード入力を配線する>[1]

        let keydownHandler: ((e: KeyboardEvent) => void) | null = null
        const originalWindow = globalThis.window
        const originalDocument = globalThis.document

        const mockWindow = {
            addEventListener: (_type: string, listener: (e: KeyboardEvent) => void) => {
                keydownHandler = listener
            },
            removeEventListener: () => {
                keydownHandler = null
            },
        }
        const mockDocument = { activeElement: null } as unknown as Document

        globalThis.window = mockWindow as unknown as Window & typeof globalThis
        globalThis.document = mockDocument

        const 解除 = 楽曲編集キーボード入力を配線する(UI状態, 操作, () => { 同期回数++ })
        assert.ok(keydownHandler !== null, 'リスナーが登録されること')

        // Ctrl+Z を送る
        let prevented = false
        keydownHandler({
            ctrlKey: true,
            metaKey: false,
            key: 'z',
            preventDefault: () => { prevented = true },
        } as unknown as KeyboardEvent)

        assert.strictEqual(取り消し回数, 1, '取り消しが呼ばれること')
        assert.strictEqual(prevented, true, 'preventDefaultが呼ばれること')

        // Alt を送る
        assert.strictEqual(UI状態.進行の外モードか, false)
        keydownHandler({
            ctrlKey: false,
            metaKey: false,
            key: 'Alt',
            repeat: false,
            preventDefault: () => {},
        } as unknown as KeyboardEvent)

        assert.strictEqual(UI状態.進行の外モードか, true, '進行の外モードがtrueに反転すること')
        assert.strictEqual(同期回数, 1, '同期が呼ばれること')

        // 入力欄にフォーカスがあるときは無視されること
        globalThis.document = { activeElement: { tagName: 'INPUT' } } as unknown as Document
        keydownHandler({
            ctrlKey: true,
            metaKey: false,
            key: 'z',
            preventDefault: () => {},
        } as unknown as KeyboardEvent)
        assert.strictEqual(取り消し回数, 1, '入力欄フォーカス中は取り消しが走らないこと')

        keydownHandler({
            ctrlKey: false,
            metaKey: false,
            key: 'Alt',
            repeat: false,
            preventDefault: () => {},
        } as unknown as KeyboardEvent)
        assert.strictEqual(UI状態.進行の外モードか, true, '入力欄フォーカス中はAltも無視されること')

        解除()
        assert.strictEqual(keydownHandler, null, 'リスナーが解除されること')

        globalThis.window = originalWindow
        globalThis.document = originalDocument
    })
})

