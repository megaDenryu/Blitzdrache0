import { describe, it, beforeEach } from 'node:test'
import assert from 'node:assert/strict'
import { テーマの控えを書く, テーマの控えを読む, テーマの控えを消す } from './テーマ保管.ts'

describe('テーマ保管の永続化テスト', () => {
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

    it('控えがないときは「控えなし」を返すこと', () => {
        const 結果 = テーマの控えを読む()
        assert.deepEqual(結果, { 種別: '控えなし' })
    })

    it('工房テーマを書いて正しく読めること', () => {
        テーマの控えを書く('工房')
        const 結果 = テーマの控えを読む()
        assert.deepEqual(結果, { 種別: '控えあり', テーマ名: '工房' })
    })

    it('夜間テーマを書いて正しく読めること', () => {
        テーマの控えを書く('夜間')
        const 結果 = テーマの控えを読む()
        assert.deepEqual(結果, { 種別: '控えあり', テーマ名: '夜間' })
    })

    it('控えを消すと控えなしになること', () => {
        テーマの控えを書く('工房')
        テーマの控えを消す()
        const 結果 = テーマの控えを読む()
        assert.deepEqual(結果, { 種別: '控えなし' })
    })

    it('不正なテーマ名が書かれていたときは形式不正を返すこと', () => {
        // @ts-expect-error 不正値注入
        テーマの控えを書く('未知のテーマ')
        const 結果 = テーマの控えを読む()
        assert.equal(結果.種別, '形式不正')
    })
})
