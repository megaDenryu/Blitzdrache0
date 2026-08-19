import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { ポインタ操作の割り当てを決める } from './ポインタ操作割り当て.ts'

describe('ポインタ操作の割り当てを決める', () => {
    it('右ボタン(2)はモードによらず常にカメラ回転になること', () => {
        assert.strictEqual(ポインタ操作の割り当てを決める(2, true), 'カメラ回転')
        assert.strictEqual(ポインタ操作の割り当てを決める(2, false), 'カメラ回転')
    })

    it('中ボタン(1)はモードによらず常にカメラ平行移動になること', () => {
        assert.strictEqual(ポインタ操作の割り当てを決める(1, true), 'カメラ平行移動')
        assert.strictEqual(ポインタ操作の割り当てを決める(1, false), 'カメラ平行移動')
    })

    it('左ボタン(0)はカメラモードのときカメラ回転になること', () => {
        assert.strictEqual(ポインタ操作の割り当てを決める(0, true), 'カメラ回転')
    })

    it('左ボタン(0)はカメラモード以外のときそのモードの主作業(造成の筆致等)になること', () => {
        assert.strictEqual(ポインタ操作の割り当てを決める(0, false), '主作業')
    })
})
