import {
    Group,
    Mesh,
    BoxGeometry,
    ConeGeometry,
    CylinderGeometry,
    MeshStandardMaterial,
} from 'three'
import type { 建物種別 } from '../../../../../生成/編集資源契約.ts'

// 各種建物の簡易プレハブ形状（家屋・塔・宝箱）をGroupとして生成する。
export function 建物グループを生成する(種別: 建物種別): Group {
    const グループ = new Group()

    if (種別 === '家屋') {
        const 土台幾何 = new BoxGeometry(10, 6, 8)
        const 土台材質 = new MeshStandardMaterial({ color: 0xca8a04, roughness: 0.7 })
        const 土台 = new Mesh(土台幾何, 土台材質)
        土台.position.y = 3

        const 屋根幾何 = new ConeGeometry(7.5, 4, 4)
        屋根幾何.rotateY(Math.PI / 4)
        const 屋根材質 = new MeshStandardMaterial({ color: 0x991b1b, roughness: 0.8 })
        const 屋根 = new Mesh(屋根幾何, 屋根材質)
        屋根.position.y = 8

        グループ.add(土台)
        グループ.add(屋根)
    } else if (種別 === '塔') {
        const 塔幾何 = new CylinderGeometry(3.5, 4, 16, 8)
        const 塔材質 = new MeshStandardMaterial({ color: 0x475569, roughness: 0.6 })
        const 塔 = new Mesh(塔幾何, 塔材質)
        塔.position.y = 8
        グループ.add(塔)
    } else if (種別 === '宝箱') {
        const 箱幾何 = new BoxGeometry(2, 1.5, 1.5)
        const 箱材質 = new MeshStandardMaterial({ color: 0xd97706, roughness: 0.4 })
        const 箱 = new Mesh(箱幾何, 箱材質)
        箱.position.y = 0.75
        グループ.add(箱)
    }

    return グループ
}
