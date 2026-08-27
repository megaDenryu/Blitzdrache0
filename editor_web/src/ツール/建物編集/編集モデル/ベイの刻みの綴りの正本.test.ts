import { deepStrictEqual } from 'node:assert/strict'
import { readFileSync, readdirSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, it } from 'node:test'

// ブラウザは実部品のモデルを配信せず、ベイの刻みと階の高さの綴りを部品カタログの写しとして持つ。
// 写しが2つになると片方だけを直した食い違いが絵の中でしか見えないため、綴りが1ファイルに限ることを機械で守る。
describe('ベイの刻みの綴りの正本', () => {
    const 建物編集の根 = dirname(dirname(fileURLToPath(import.meta.url)))

    it('2.0と2.6の綴りはベイの寸法だけが持つ', () => {
        const 綴りを持つファイル = tsファイルを数え上げる(建物編集の根).filter((パス) => {
            const 本文 = readFileSync(パス, 'utf8')
            return /(^|[^.\d])2\.6(?![\d])/.test(本文) || /(^|[^.\d])2\.0(?![\d])/.test(本文)
        })
        deepStrictEqual(
            綴りを持つファイル.map((パス) => パス.split(/[\\/]/).pop()),
            ['ベイの寸法.ts'],
        )
    })

    function tsファイルを数え上げる(ディレクトリ: string): string[] {
        return readdirSync(ディレクトリ, { withFileTypes: true }).flatMap((項目) => {
            const パス = join(ディレクトリ, 項目.name)
            if (項目.isDirectory()) return tsファイルを数え上げる(パス)
            return 項目.name.endsWith('.ts') && !項目.name.endsWith('.test.ts') ? [パス] : []
        })
    }
})
