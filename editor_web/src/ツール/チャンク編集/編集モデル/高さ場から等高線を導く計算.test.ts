import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 高さ場から等高線を導く } from './高さ場から等高線を導く計算.ts'
import { 等高線から高さ場を生成する } from './等高線から高さ場を生成する計算.ts'

const 解像度 = 65
const 一辺 = 64
const 格子間隔 = 一辺 / (解像度 - 1)

// 傾いた平面は離散ラプラス方程式の解そのものであるため、等高線を導いて生成し直したときの誤差は
// 等値線の頂点の間引きと格子点への固定の丸めだけから出る。
function 傾いた平面(): Float32Array {
    const 格子 = new Float32Array(解像度 * 解像度)
    for (let gz = 0; gz < 解像度; gz++) {
        for (let gx = 0; gx < 解像度; gx++) {
            const wx = gx * 格子間隔 - 一辺 / 2
            const wz = gz * 格子間隔 - 一辺 / 2
            格子[gz * 解像度 + gx] = 0.3 * wx + 0.1 * wz
        }
    }
    return 格子
}

describe('高さ場から等高線を導く計算', () => {
    it('等高線を導いてから生成し直したときの誤差が間隔の半分以内であること', () => {
        const 間隔 = 2
        const 元 = 傾いた平面()
        const 等高線一覧 = 高さ場から等高線を導く(間隔, 解像度, 一辺, 元)
        assert.ok(等高線一覧.length >= 10, `間隔${間隔}mの等高線が10本以上出るべき: ${等高線一覧.length}`)
        for (const 線 of 等高線一覧) {
            assert.strictEqual(Math.abs(線.高さメートル % 間隔), 0)
            assert.ok(線.頂点列.length >= 2)
            assert.strictEqual(線.閉じている, false, '平面の等高線は外周で切れるため開いているべき')
        }
        const 外周だけ残した格子 = new Float32Array(解像度 * 解像度)
        for (let i = 0; i < 解像度; i++) {
            for (const 添字 of [i, (解像度 - 1) * 解像度 + i, i * 解像度, i * 解像度 + 解像度 - 1]) {
                外周だけ残した格子[添字] = 元[添字] ?? 0
            }
        }
        const 生成し直し = 等高線から高さ場を生成する(等高線一覧, 解像度, 一辺, 外周だけ残した格子)
        let 最大誤差 = 0
        for (let i = 0; i < 元.length; i++) {
            最大誤差 = Math.max(最大誤差, Math.abs((生成し直し[i] ?? 0) - (元[i] ?? 0)))
        }
        assert.ok(最大誤差 <= 間隔 / 2, `最大誤差は間隔の半分以内であるべき: ${最大誤差}`)
    })

    it('丘の周りの等値線は閉じた等高線になること', () => {
        const 格子 = new Float32Array(解像度 * 解像度)
        for (let gz = 0; gz < 解像度; gz++) {
            for (let gx = 0; gx < 解像度; gx++) {
                const wx = gx * 格子間隔 - 一辺 / 2
                const wz = gz * 格子間隔 - 一辺 / 2
                格子[gz * 解像度 + gx] = Math.max(0, 10 - Math.hypot(wx, wz) / 2)
            }
        }
        const 等高線一覧 = 高さ場から等高線を導く(2, 解像度, 一辺, 格子)
        const 高さ6の線 = 等高線一覧.filter((線) => 線.高さメートル === 6)
        assert.strictEqual(高さ6の線.length, 1, '高さ6の等値線は1本に繋がるべき')
        assert.strictEqual(高さ6の線[0]?.閉じている, true)
        for (const 頂点 of 高さ6の線[0]?.頂点列 ?? []) {
            assert.ok(Math.abs(Math.hypot(頂点.x, 頂点.z) - 8) < 格子間隔, `高さ6の等高線は半径8mの円の上にあるべき: (${頂点.x},${頂点.z})`)
        }
    })

    it('間隔が正でなければ拒否されること', () => {
        assert.throws(() => 高さ場から等高線を導く(0, 解像度, 一辺, 傾いた平面()), /正の数/)
    })
})
