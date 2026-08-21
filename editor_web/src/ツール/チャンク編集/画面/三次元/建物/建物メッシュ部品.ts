import { Group } from 'three'
import { グループ } from 'SengenThree'
import type { 建物の管理 } from '../../../編集モデル/index.ts'
import type { 建物外形定義 } from '../../../../../生成/編集資源契約.ts'
import { 建物形状共有資源 } from './建物形状生成.ts'

// 配置されている建物の簡易形状メッシュ群を統括・更新する部品。
export class 建物メッシュ部品 extends グループ {
    private readonly _共有資源: 建物形状共有資源
    private _建物メッシュ写像: Map<string, Group> = new Map<string, Group>()

    public constructor() {
        super()
        this._共有資源 = new 建物形状共有資源(this.資源台帳)
    }

    public get 建物グループ一覧(): readonly Group[] {
        return Array.from(this._建物メッシュ写像.values())
    }

    public 識別子からグループを取得する(識別子: string): Group | undefined {
        return this._建物メッシュ写像.get(識別子)
    }

    public 建物定義一覧を更新する(定義一覧: ReadonlyArray<建物外形定義>): void {
        this._共有資源.定義一覧を更新する(定義一覧)
    }

    public 更新する(建物管理: 建物の管理): void {
        for (const グループ of this._建物メッシュ写像.values()) {
            this.実体.remove(グループ)
        }
        this._建物メッシュ写像.clear()

        const 建物一覧 = 建物管理.一覧を取得する()
        for (const 建物 of 建物一覧) {
            const 建物グループ = this._共有資源.建物グループを生成する(建物.建物定義ID)
            建物グループ.position.set(建物.位置.x, 建物.位置.y, 建物.位置.z)
            建物グループ.rotation.y = 建物.向きラジアン
            this.実体.add(建物グループ)
            this._建物メッシュ写像.set(建物.識別子, 建物グループ)
        }
    }
}
