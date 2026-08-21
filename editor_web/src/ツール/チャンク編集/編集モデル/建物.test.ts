import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 建物の管理 } from './建物.ts'
import { 高さ場 } from './高さ場.ts'

describe('建物編集モデル', () => {
    it('建物の配置・移動・削除が正常に動作すること', () => {
        const 管理 = new 建物の管理()
        管理.配置する({
            識別子: '家屋_1',
            建物定義ID: 'frame_house_one_bay',
            位置: { x: 10, y: 5, z: 10 },
            向きラジアン: 0,
            基礎半径メートル: 7.0,
            なじみ半径メートル: 14.0,
        })
        assert.strictEqual(管理.一覧を取得する().length, 1)

        // 重複配置の例外
        assert.throws(() => {
            管理.配置する({
                識別子: '家屋_1',
                建物定義ID: 'frame_house_one_bay',
                位置: { x: 0, y: 0, z: 0 },
                向きラジアン: 0,
                基礎半径メートル: 5,
                なじみ半径メートル: 10,
            })
        })

        // 移動
        管理.移動する('家屋_1', { x: 20, y: 8, z: 20 }, Math.PI / 2)
        const 取得建物 = 管理.取得する('家屋_1')
        assert.deepStrictEqual(取得建物.位置, { x: 20, y: 8, z: 20 })
        assert.strictEqual(取得建物.向きラジアン, Math.PI / 2)

        // 削除
        const 削除された建物 = 管理.削除する('家屋_1')
        assert.strictEqual(削除された建物.識別子, '家屋_1')
        assert.strictEqual(管理.一覧を取得する().length, 0)
    })

    it('建物基礎の平坦化で基礎半径内が基礎高さになりなじみ半径外は不変であること', () => {
        const 解像度 = 65
        const 一辺 = 64
        const 対象高さ場 = new 高さ場(解像度, 一辺)
        const 基礎標高 = 15.0
        const 管理 = new 建物の管理([
            {
                識別子: '塔_1',
                建物定義ID: 'frame_house_two_story',
                位置: { x: 0, y: 基礎標高, z: 0 },
                向きラジアン: 0,
                基礎半径メートル: 5.0,
                なじみ半径メートル: 12.0,
            },
        ])

        管理.基礎を平坦化する('塔_1', 対象高さ場)

        // 基礎半径内 (r=2)
        const 基礎内高さ = 対象高さ場.標本高さを取得する(2, 0)
        assert.ok(Math.abs(基礎内高さ - 基礎標高) < 1e-2, `基礎内は基礎標高と一致するべき: 実際=${基礎内高さ}`)

        // なじみ半径外 (r=20)
        const 外側高さ = 対象高さ場.標本高さを取得する(20, 0)
        assert.strictEqual(外側高さ, 0, 'なじみ半径外は0のままであるべき')
    })
})
