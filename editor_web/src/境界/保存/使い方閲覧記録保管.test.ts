import { describe, it, beforeEach } from 'node:test'
import assert from 'node:assert/strict'
import { 使い方の閲覧記録を書く, 使い方を閲覧済みか } from './使い方閲覧記録保管.ts'

describe('使い方閲覧記録保管の永続化テスト', () => {
    const モックストレージ = new Map<string, string>()

    beforeEach(() => {
        モックストレージ.clear()
        // @ts-expect-error テスト用モック
        globalThis.localStorage = {
            getItem: (key: string) => モックストレージ.get(key) ?? null,
            setItem: (key: string, value: string) => {
                モックストレージ.set(key, String(value))
            },
            removeItem: (key: string) => {
                モックストレージ.delete(key)
            },
            clear: () => {
                モックストレージ.clear()
            },
        }
    })

    it('記録がないときは未閲覧と判定されること', () => {
        assert.strictEqual(使い方を閲覧済みか(), false)
    })

    it('閲覧記録を書くと閲覧済みと判定されること', () => {
        使い方の閲覧記録を書く()
        assert.strictEqual(使い方を閲覧済みか(), true)
    })

    it('localStorageが使えない環境では未閲覧扱いになり例外を投げないこと', () => {
        // @ts-expect-error テスト用に破壊
        globalThis.localStorage = {
            getItem: () => {
                throw new Error('使用不可')
            },
            setItem: () => {
                throw new Error('使用不可')
            },
        }
        assert.doesNotThrow(() => 使い方の閲覧記録を書く())
        assert.strictEqual(使い方を閲覧済みか(), false)
    })
})
