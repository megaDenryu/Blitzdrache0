import { 建物定義IDを生成する, type 建物の格子接続 } from '../../境界/通信/index.ts'
import type { エクスプローラーパネル } from '../エクスプローラー/index.ts'
import { 建物定義IDを発番する, 新しい建物の既定の表示名 } from '../建物定義IDを発番する.ts'
import type { タブ開閉サービス } from './タブ開閉サービス.ts'

// 保管庫の接続とエクスプローラーとタブ開閉を保持し、建物の一覧の読み直しと新規作成を公開する操作サービス。
// 一覧の正本はサーバーの台帳であり、ブラウザ側は写しを持たない。
export class 建物の一覧サービス {
    public constructor(
        private readonly _接続: 建物の格子接続,
        private readonly _エクスプローラー: エクスプローラーパネル,
        private readonly _タブ開閉: タブ開閉サービス,
    ) {}

    public async 読み直す(): Promise<void> {
        const 結果 = await this._接続.建物一覧を読む()
        if (結果.種別 === '成功') this._エクスプローラー.建物の一覧を作り直す(結果.値)
    }

    // 新しい建物へ識別子を1つ発番してタブを開く。正本ができるのは、そのタブで保存したときである。
    // 一覧を先に読むのは、既にある識別子と重ならない番号を選ぶためである。
    public async 新しい建物を作る(): Promise<void> {
        const 結果 = await this._接続.建物一覧を読む()
        const 既にある識別子一覧 = 結果.種別 === '成功' ? 結果.値.map((項目) => 建物定義IDを生成する(項目.識別子)) : []
        this._タブ開閉.建物を開く(建物定義IDを発番する(既にある識別子一覧), 新しい建物の既定の表示名())
    }
}
