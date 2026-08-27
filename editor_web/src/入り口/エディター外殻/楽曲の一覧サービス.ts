import type { 楽曲接続 } from '../../境界/通信/index.ts'
import type { 楽曲エクスプローラー } from '../エクスプローラー/index.ts'
import { 楽曲IDを発番する, 新しい楽曲の既定の表示名 } from '../楽曲IDを発番する.ts'
import type { タブ開閉サービス } from './タブ開閉サービス.ts'

// 保管庫の楽曲接続と楽曲エクスプローラーとタブ開閉を保持し、楽曲一覧の読み直しと新規作成を担うサービス。
export class 楽曲の一覧サービス {
    public constructor(
        private readonly _接続: 楽曲接続,
        private readonly _エクスプローラー: 楽曲エクスプローラー,
        private readonly _タブ開閉: タブ開閉サービス,
    ) {}

    public async 読み直す(): Promise<void> {
        const 結果 = await this._接続.楽曲一覧を読む()
        if (結果.種別 === '成功') this._エクスプローラー.一覧を作り直す(結果.値)
    }

    // 新しい楽曲へ名乗りを1つ発番してタブを開く。正本ができるのは保存時である。
    public async 新しい楽曲を作る(): Promise<void> {
        const 結果 = await this._接続.楽曲一覧を読む()
        const 既にある名乗り一覧 = 結果.種別 === '成功' ? 結果.値 : []
        this._タブ開閉.楽曲を開く(楽曲IDを発番する(既にある名乗り一覧), 新しい楽曲の既定の表示名())
    }
}

