import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 注視点マーカー表示制御器 } from './注視点マーカー表示制御器.ts'
import type { 注視点表示先, 遅延実行 } from './注視点マーカー表示制御器.ts'

class 記録用マーカー implements 注視点表示先 {
    表示履歴: { x: number; y: number; z: number }[] = []
    非表示回数: number = 0

    地点へ表示する(x: number, y: number, z: number): this {
        this.表示履歴.push({ x, y, z })
        return this
    }

    非表示にする(): this {
        this.非表示回数 += 1
        return this
    }
}

// 実時間を進めず、予約されたコールバックをテストから手動で発火できる偽の遅延実行。
class 手動遅延実行 implements 遅延実行 {
    private _次の識別子: number = 1
    private readonly _保留一覧: Map<number, () => void> = new Map()
    取り消し履歴: number[] = []

    予約する(_遅延ミリ秒: number, コールバック: () => void): number {
        const 識別子 = this._次の識別子++
        this._保留一覧.set(識別子, コールバック)
        return 識別子
    }

    取り消す(識別子: number): void {
        this.取り消し履歴.push(識別子)
        this._保留一覧.delete(識別子)
    }

    保留数(): number {
        return this._保留一覧.size
    }

    すべて発火する(): void {
        for (const コールバック of this._保留一覧.values()) {
            コールバック()
        }
        this._保留一覧.clear()
    }
}

describe('注視点マーカー表示制御器', () => {
    it('操作されたらマーカーを指定位置へ表示すること', () => {
        const マーカー = new 記録用マーカー()
        const タイマー = new 手動遅延実行()
        const 制御器 = new 注視点マーカー表示制御器(マーカー, タイマー)

        制御器.操作された(1, 2, 3)

        assert.deepStrictEqual(マーカー.表示履歴, [{ x: 1, y: 2, z: 3 }])
        assert.strictEqual(マーカー.非表示回数, 0)
    })

    it('猶予時間の経過(タイマー発火)で非表示になること', () => {
        const マーカー = new 記録用マーカー()
        const タイマー = new 手動遅延実行()
        const 制御器 = new 注視点マーカー表示制御器(マーカー, タイマー)

        制御器.操作された(0, 0, 0)
        タイマー.すべて発火する()

        assert.strictEqual(マーカー.非表示回数, 1)
    })

    it('猶予時間内の再操作でタイマーが取り消され再設定されること(発火しても1回だけ非表示)', () => {
        const マーカー = new 記録用マーカー()
        const タイマー = new 手動遅延実行()
        const 制御器 = new 注視点マーカー表示制御器(マーカー, タイマー)

        制御器.操作された(0, 0, 0)
        assert.strictEqual(タイマー.保留数(), 1)

        制御器.操作された(1, 1, 1)
        assert.strictEqual(タイマー.取り消し履歴.length, 1, '再操作で前回のタイマーが取り消されるべき')
        assert.strictEqual(タイマー.保留数(), 1, '新しいタイマーが1つだけ保留されているべき')

        タイマー.すべて発火する()
        assert.strictEqual(マーカー.非表示回数, 1)
    })

    it('破棄するで保留中のタイマーを取り消すこと', () => {
        const マーカー = new 記録用マーカー()
        const タイマー = new 手動遅延実行()
        const 制御器 = new 注視点マーカー表示制御器(マーカー, タイマー)

        制御器.操作された(0, 0, 0)
        制御器.破棄する()

        assert.strictEqual(タイマー.保留数(), 0)
        タイマー.すべて発火する()
        assert.strictEqual(マーカー.非表示回数, 0, '破棄後はコールバックが発火しても非表示処理は起きないべき')
    })
})
