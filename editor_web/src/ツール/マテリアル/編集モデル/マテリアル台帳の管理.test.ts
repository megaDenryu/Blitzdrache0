import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { マテリアル台帳の管理 } from './マテリアル台帳の管理.ts'
import { 既定の台帳を組み立てる } from './既定の台帳を組み立てる.ts'

describe('マテリアル台帳の管理', () => {
    it('既定の台帳は4材質の自層割当で検証を通る形であること', () => {
        const 台帳 = 既定の台帳を組み立てる()
        assert.strictEqual(台帳.マテリアル一覧.length, 4)
        assert.strictEqual(台帳.層割当.草, '草')
        assert.strictEqual(台帳.層割当.砂, '砂')
    })

    it('材質を追加すると一覧が1件増えること', () => {
        const 管理 = new マテリアル台帳の管理()
        const 元件数 = 管理.台帳を取得する().マテリアル一覧.length
        管理.材質を追加する()
        assert.strictEqual(管理.台帳を取得する().マテリアル一覧.length, 元件数 + 1)
    })

    it('材質名を変更するとその名前を参照していた層割当も追従すること', () => {
        const 管理 = new マテリアル台帳の管理()
        assert.strictEqual(管理.台帳を取得する().層割当.草, '草')
        管理.材質名を変更する(0, '芝草')
        const 台帳 = 管理.台帳を取得する()
        assert.strictEqual(台帳.マテリアル一覧[0]?.エンジン材質名, '芝草')
        assert.strictEqual(台帳.層割当.草, '芝草')
    })

    it('参照されている材質を削除すると層割当が残った材質へ差し替わること', () => {
        const 管理 = new マテリアル台帳の管理()
        管理.材質を削除する(0)
        const 台帳 = 管理.台帳を取得する()
        assert.strictEqual(台帳.マテリアル一覧.length, 3)
        assert.notStrictEqual(台帳.層割当.草, '草')
        assert.strictEqual(台帳.層割当.草, 台帳.マテリアル一覧[0]?.エンジン材質名)
    })

    it('識別色を変更しても他フィールドは変わらないこと', () => {
        const 管理 = new マテリアル台帳の管理()
        管理.識別色を変更する(1, '#123456')
        assert.strictEqual(管理.台帳を取得する().マテリアル一覧[1]?.識別色, '#123456')
        assert.strictEqual(管理.台帳を取得する().マテリアル一覧[1]?.エンジン材質名, '泥')
    })

    it('状態を上書きすると内部の台帳が完全に差し替わり複製であること', () => {
        const 管理 = new マテリアル台帳の管理()
        const 新台帳 = {
            マテリアル一覧: [{ エンジン材質名: '溶岩', 識別色: '#ff4500' }],
            層割当: { 草: '溶岩', 泥: '溶岩', 岩: '溶岩', 砂: '溶岩' },
        }
        管理.状態を上書きする(新台帳)
        const 取得 = 管理.台帳を取得する()
        assert.deepStrictEqual(取得, 新台帳)
        assert.notStrictEqual(取得, 新台帳)
    })
})
