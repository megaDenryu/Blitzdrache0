import { BoxGeometry, Group, Mesh, MeshBasicMaterial, MeshStandardMaterial } from 'three'
import type { 建物外形定義 } from '../../../../../生成/編集資源契約.ts'
import type { 破棄可能資源, 資源台帳 } from 'SengenThree'

// コンパイラが実部品の展開から算出した外接箱を、編集画面の配置外形へ写す共有資源。
export class 建物形状共有資源 implements 破棄可能資源 {
    private readonly _幾何: Map<string, BoxGeometry> = new Map()
    private readonly _定義: Map<string, 建物外形定義> = new Map()
    private readonly _材質: MeshStandardMaterial
    private readonly _未解決幾何: BoxGeometry
    private readonly _未解決材質: MeshBasicMaterial

    public constructor(private readonly _台帳: 資源台帳) {
        this._材質 = new MeshStandardMaterial({ color: 0xca8a04, roughness: 0.72, transparent: true, opacity: 0.78 })
        this._未解決幾何 = new BoxGeometry(2, 2, 2)
        this._未解決材質 = new MeshBasicMaterial({ color: 0xff0000, wireframe: true })
        this._台帳.登録する(this._材質)
        this._台帳.登録する(this._未解決幾何)
        this._台帳.登録する(this._未解決材質)
    }

    public 定義一覧を更新する(定義一覧: ReadonlyArray<建物外形定義>): void {
        for (const 定義 of 定義一覧) {
            if (this._幾何.has(定義.識別子)) continue
            const 最小 = 定義.外接箱.最小
            const 最大 = 定義.外接箱.最大
            const 幾何 = new BoxGeometry(最大[0] - 最小[0], 最大[1] - 最小[1], 最大[2] - 最小[2])
            this._幾何.set(定義.識別子, 幾何)
            this._定義.set(定義.識別子, 定義)
            this._台帳.登録する(幾何)
        }
    }

    public 建物グループを生成する(建物定義ID: string): Group {
        const グループ = new Group()
        const 定義 = this._定義.get(建物定義ID)
        const 幾何 = this._幾何.get(建物定義ID)
        if (定義 === undefined || 幾何 === undefined) {
            const 代替外形 = new Mesh(this._未解決幾何, this._未解決材質)
            代替外形.position.y = 1
            グループ.add(代替外形)
            return グループ
        }
        const 最小 = 定義.外接箱.最小
        const 最大 = 定義.外接箱.最大
        const 外形 = new Mesh(幾何, this._材質)
        外形.position.set((最小[0] + 最大[0]) * 0.5, (最小[1] + 最大[1]) * 0.5, (最小[2] + 最大[2]) * 0.5)
        グループ.add(外形)
        return グループ
    }

    public 建物定義があるか(建物定義ID: string): boolean {
        return this._定義.has(建物定義ID)
    }

    public dispose(): void {}
}
