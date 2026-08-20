import type { 編集コマンド, チャンク座標 } from '../../../生成/編集資源契約.ts'
import type { プロジェクト保管庫接続 } from '../../../境界/通信/index.ts'
import { チャンク座標キーを生成する } from '../../../境界/通信/index.ts'
import type { ワールド編集状態 } from '../編集モデル/index.ts'
import { コマンドが触るチャンク座標を求める } from './コマンドが触るチャンク座標.ts'
import { 編集コマンドを適用する } from './編集コマンドの適用.ts'
import { ワールド永続化サービス } from './永続化サービス.ts'

// 保管庫から資源を読み、契約16枝の同一TS適用実装でコマンド列を適用し、検証付きで保存するサービス。
// ヘッドレス側とブラウザ側チャンク編集ツールの両方が、この1つを注入して同じ実装を共有する。
export class 編集コマンド一括適用サービス {
    private readonly _永続化: ワールド永続化サービス

    public constructor(保管庫: プロジェクト保管庫接続) {
        this._永続化 = new ワールド永続化サービス(保管庫)
    }

    public async 一括適用する(コマンド列: readonly 編集コマンド[]): Promise<number> {
        const 対象チャンク座標一覧 = this._コマンド列からチャンク座標一覧を抽出する(コマンド列)
        const 状態 = await this._永続化.複数チャンクを読み込む(対象チャンク座標一覧)

        for (const コマンド of コマンド列) {
            this._チャンク座標を同期する(状態, コマンド)
            編集コマンドを適用する(状態, コマンド)
        }

        await this._永続化.保存する(状態)
        return コマンド列.length
    }

    private _チャンク座標を同期する(状態: ワールド編集状態, コマンド: 編集コマンド): void {
        const 座標 = コマンドが触るチャンク座標を求める(コマンド)
        if (座標 !== null) {
            状態.選択中チャンク座標 = 座標
        }
    }

    private _コマンド列からチャンク座標一覧を抽出する(
        コマンド列: readonly 編集コマンド[],
    ): readonly チャンク座標[] {
        const 原点: チャンク座標 = { x: 0, z: 0 }
        const マップ = new Map<string, チャンク座標>()
        マップ.set(チャンク座標キーを生成する(原点), 原点)
        for (const コマンド of コマンド列) {
            const 座標 = コマンドが触るチャンク座標を求める(コマンド)
            if (座標 !== null) {
                マップ.set(チャンク座標キーを生成する(座標), 座標)
            }
        }
        return Array.from(マップ.values())
    }
}
