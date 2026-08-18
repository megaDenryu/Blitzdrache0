import {
    Group,
    Mesh,
    BoxGeometry,
    ConeGeometry,
    CylinderGeometry,
    MeshStandardMaterial,
} from 'three'
import type { 建物種別 } from '../../../../../生成/編集資源契約.ts'
import type { 破棄可能資源, 資源台帳 } from 'SengenThree'

// 建物の共通ジオメトリとマテリアルを保持し、毎回の再生成を防ぐ共有資源。
export class 建物形状共有資源 implements 破棄可能資源 {
    public readonly 家屋土台幾何: BoxGeometry
    public readonly 家屋土台材質: MeshStandardMaterial
    public readonly 家屋屋根幾何: ConeGeometry
    public readonly 家屋屋根材質: MeshStandardMaterial
    public readonly 塔幾何: CylinderGeometry
    public readonly 塔材質: MeshStandardMaterial
    public readonly 宝箱幾何: BoxGeometry
    public readonly 宝箱材質: MeshStandardMaterial

    public constructor(台帳: 資源台帳) {
        this.家屋土台幾何 = new BoxGeometry(10, 6, 8)
        this.家屋土台材質 = new MeshStandardMaterial({ color: 0xca8a04, roughness: 0.7 })
        this.家屋屋根幾何 = new ConeGeometry(7.5, 4, 4)
        this.家屋屋根幾何.rotateY(Math.PI / 4)
        this.家屋屋根材質 = new MeshStandardMaterial({ color: 0x991b1b, roughness: 0.8 })
        this.塔幾何 = new CylinderGeometry(3.5, 4, 16, 8)
        this.塔材質 = new MeshStandardMaterial({ color: 0x475569, roughness: 0.6 })
        this.宝箱幾何 = new BoxGeometry(2, 1.5, 1.5)
        this.宝箱材質 = new MeshStandardMaterial({ color: 0xd97706, roughness: 0.4 })

        台帳.登録する(this.家屋土台幾何)
        台帳.登録する(this.家屋土台材質)
        台帳.登録する(this.家屋屋根幾何)
        台帳.登録する(this.家屋屋根材質)
        台帳.登録する(this.塔幾何)
        台帳.登録する(this.塔材質)
        台帳.登録する(this.宝箱幾何)
        台帳.登録する(this.宝箱材質)
    }

    public dispose(): void {}
}

// 共有資源を用いて各種建物の簡易プレハブ形状をGroupとして生成する。
export function 建物グループを生成する(種別: 建物種別, 資源: 建物形状共有資源): Group {
    const グループ = new Group()

    if (種別 === '家屋') {
        const 土台 = new Mesh(資源.家屋土台幾何, 資源.家屋土台材質)
        土台.position.y = 3
        const 屋根 = new Mesh(資源.家屋屋根幾何, 資源.家屋屋根材質)
        屋根.position.y = 8
        グループ.add(土台)
        グループ.add(屋根)
    } else if (種別 === '塔') {
        const 塔 = new Mesh(資源.塔幾何, 資源.塔材質)
        塔.position.y = 8
        グループ.add(塔)
    } else if (種別 === '宝箱') {
        const 箱 = new Mesh(資源.宝箱幾何, 資源.宝箱材質)
        箱.position.y = 0.75
        グループ.add(箱)
    }

    return グループ
}
