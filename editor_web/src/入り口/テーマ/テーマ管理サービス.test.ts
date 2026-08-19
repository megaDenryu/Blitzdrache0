import { describe, it, beforeEach } from 'node:test'
import assert from 'node:assert/strict'
import { テーマ管理サービス } from './テーマ管理サービス.ts'
import { 工房テーマ, 夜間テーマ } from './テーマ定義一覧.ts'

describe('テーマ管理サービスのテスト', () => {
    const モックストレージ = new Map<string, string>()

    beforeEach(() => {
        モックストレージ.clear()
        // @ts-expect-error モック
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

    it('初回起動時は既定の「工房」テーマになること', () => {
        const サービス = new テーマ管理サービス()
        assert.equal(サービス.現在テーマを取得する().名前, '工房')
        assert.deepEqual(サービス.現在テーマを取得する(), 工房テーマ)
    })

    it('保存済みのテーマがある場合はそれを復元すること', () => {
        モックストレージ.set('BLITZDRACHE_EDITOR_THEME', '夜間')
        const サービス = new テーマ管理サービス()
        assert.equal(サービス.現在テーマを取得する().名前, '夜間')
        assert.deepEqual(サービス.現在テーマを取得する(), 夜間テーマ)
    })

    it('テーマを変更するとリスナーへ通知されlocalStorageへ保存されること', () => {
        const サービス = new テーマ管理サービス()
        let 通知されたテーマ名: string | null = null

        const 解除 = サービス.onテーマ変更((テーマ) => {
            通知されたテーマ名 = テーマ.名前
        })

        サービス.テーマを変更する('夜間')
        assert.equal(サービス.現在テーマを取得する().名前, '夜間')
        assert.equal(通知されたテーマ名, '夜間')
        assert.equal(モックストレージ.get('BLITZDRACHE_EDITOR_THEME'), '夜間')

        解除()
        サービス.テーマを変更する('工房')
        assert.equal(サービス.現在テーマを取得する().名前, '工房')
        // リスナー解除後は古い値のまま
        assert.equal(通知されたテーマ名, '夜間')
        assert.equal(モックストレージ.get('BLITZDRACHE_EDITOR_THEME'), '工房')
    })
})
