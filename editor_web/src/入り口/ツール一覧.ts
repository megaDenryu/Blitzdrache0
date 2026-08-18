import type { プロジェクト保管庫接続 } from '../境界/通信/index.ts'
import { ワールドパイプラインエディター } from '../ツール/ワールド/index.ts'
import { 大域グリッドエディター } from '../ツール/大域グリッド/index.ts'
import type { ツール項目 } from './ツール定義.ts'

// エディター外殻が管理する利用可能ツール一覧の定義。
export function ツール登録一覧を生成する(保管庫: プロジェクト保管庫接続): readonly ツール項目[] {
    return [
        {
            識別子: 'world-pipeline',
            ラベル: 'ワールド',
            アイコン記号: 'W',
            ツールを生成する: () => new ワールドパイプラインエディター(undefined, 保管庫),
        },
        {
            識別子: 'global-grid',
            ラベル: '大域グリッド',
            アイコン記号: 'G',
            ツールを生成する: () => new 大域グリッドエディター(undefined, 保管庫),
        },
    ]
}
