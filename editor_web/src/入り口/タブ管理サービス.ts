import type { 実行可能ツール } from './ツール定義.ts'

// エディタエリアに開かれた複数ツールの寿命と前面・背面ライフサイクルを統括する。
export class タブ管理サービス {
    private readonly _ツールマップ = new Map<string, 実行可能ツール>()
    private _前面ツールID: string | null = null

    public ツールを登録する(タブID: string, ツール: 実行可能ツール): void {
        this._ツールマップ.set(タブID, ツール)
    }

    public タブを選択する(タブID: string, 幅?: number, 高さ?: number): 実行可能ツール | undefined {
        if (this._前面ツールID === タブID) {
            return this._ツールマップ.get(タブID)
        }
        if (this._前面ツールID !== null) {
            this._ツールマップ.get(this._前面ツールID)?.背面になった()
        }
        const 新ツール = this._ツールマップ.get(タブID)
        新ツール?.前面になった()
        if (幅 !== undefined && 高さ !== undefined) {
            新ツール?.寸法を合わせる(幅, 高さ)
        } else if (typeof window !== 'undefined') {
            新ツール?.寸法を合わせる(window.innerWidth, window.innerHeight)
        }
        this._前面ツールID = タブID
        return 新ツール
    }

    public タブを破棄する(タブID: string): void {
        const ツール = this._ツールマップ.get(タブID)
        if (ツール !== undefined) {
            ツール.背面になった()
            ツール.delete()
            this._ツールマップ.delete(タブID)
        }
        if (this._前面ツールID === タブID) {
            this._前面ツールID = null
        }
    }

    public 前面ツールを取得する(): 実行可能ツール | undefined {
        return this._前面ツールID !== null ? this._ツールマップ.get(this._前面ツールID) : undefined
    }

    public 全て破棄する(): void {
        for (const ツール of this._ツールマップ.values()) {
            ツール.delete()
        }
        this._ツールマップ.clear()
        this._前面ツールID = null
    }
}
