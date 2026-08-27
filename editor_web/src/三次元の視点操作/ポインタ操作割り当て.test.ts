import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { ポインタ操作の割り当てを決める } from './ポインタ操作割り当て.ts'

describe('ポインタ操作の割り当てを決める', () => {
    it('右ボタン(2)は常にカメラ回転になること', () => {
        assert.strictEqual(ポインタ操作の割り当てを決める(2), 'カメラ回転')
    })

    it('中ボタン(1)は常にカメラ平行移動になること', () => {
        assert.strictEqual(ポインタ操作の割り当てを決める(1), 'カメラ平行移動')
    })

    it('左ボタン(0)は常にそのモードの主作業(選択・造成の筆致等)になること', () => {
        assert.strictEqual(ポインタ操作の割り当てを決める(0), '主作業')
    })
})
