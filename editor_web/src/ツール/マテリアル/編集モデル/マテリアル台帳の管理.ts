import type { マテリアル台帳, マテリアル定義, 地表材質層 } from '../../../生成/編集資源契約.ts'
import { 既定の台帳を組み立てる } from './既定の台帳を組み立てる.ts'

export const 地表材質層一覧: readonly 地表材質層[] = ['草', '泥', '岩', '砂']

// マテリアル台帳の一覧編集・層割当変更を保持する編集モデル。保存前の変更はこの型の
// フィールドだけに閉じ、読込/保存のたびに丸ごと上書きする(取り消し履歴は持たない)。
export class マテリアル台帳の管理 {
    private _台帳: マテリアル台帳

    public constructor(初期台帳: マテリアル台帳 = 既定の台帳を組み立てる()) {
        this._台帳 = 複製する(初期台帳)
    }

    public 台帳を取得する(): マテリアル台帳 {
        return 複製する(this._台帳)
    }

    public 状態を上書きする(台帳: マテリアル台帳): void {
        this._台帳 = 複製する(台帳)
    }

    public 材質を追加する(): マテリアル定義 {
        const 新規材質: マテリアル定義 = {
            エンジン材質名: `新規材質${this._台帳.マテリアル一覧.length + 1}`,
            識別色: '#888888',
        }
        this._台帳.マテリアル一覧.push(新規材質)
        return { ...新規材質 }
    }

    public 材質名を変更する(添字: number, 新しい材質名: string): void {
        const 対象 = this._台帳.マテリアル一覧[添字]
        if (対象 === undefined) return
        const 旧材質名 = 対象.エンジン材質名
        対象.エンジン材質名 = 新しい材質名
        this._層割当の参照を差し替える(旧材質名, 新しい材質名)
    }

    public 識別色を変更する(添字: number, 新しい識別色: string): void {
        const 対象 = this._台帳.マテリアル一覧[添字]
        if (対象 === undefined) return
        対象.識別色 = 新しい識別色
    }

    public 材質を削除する(添字: number): void {
        const 対象 = this._台帳.マテリアル一覧[添字]
        if (対象 === undefined) return
        this._台帳.マテリアル一覧.splice(添字, 1)
        const 代替材質名 = this._台帳.マテリアル一覧[0]?.エンジン材質名 ?? ''
        this._層割当の参照を差し替える(対象.エンジン材質名, 代替材質名)
    }

    public 層の割当を変更する(層: 地表材質層, 材質名: string): void {
        this._台帳.層割当[層] = 材質名
    }

    private _層割当の参照を差し替える(旧材質名: string, 新材質名: string): void {
        for (const 層 of 地表材質層一覧) {
            if (this._台帳.層割当[層] === 旧材質名) {
                this._台帳.層割当[層] = 新材質名
            }
        }
    }
}

function 複製する(台帳: マテリアル台帳): マテリアル台帳 {
    return {
        マテリアル一覧: 台帳.マテリアル一覧.map((定義) => ({ ...定義 })),
        層割当: { ...台帳.層割当 },
    }
}
