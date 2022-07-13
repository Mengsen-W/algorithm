/*
 * @Date: 2022-07-13
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-13
 * @FilePath: /algorithm/735_asteroid_collision/asteroid_collision.go
 */

package main

import "reflect"

func asteroidCollision(asteroids []int) (st []int) {
	for _, aster := range asteroids {
		alive := true
		for alive && aster < 0 && len(st) > 0 && st[len(st)-1] > 0 {
			alive = st[len(st)-1] < -aster // aster 是否存在
			if st[len(st)-1] <= -aster {   // 栈顶行星爆炸
				st = st[:len(st)-1]
			}
		}
		if alive {
			st = append(st, aster)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		asteroids := []int{5, 10, -5}
		ans := []int{5, 10}
		assert(asteroidCollision(asteroids), ans)
	}

	{
		asteroids := []int{8, -8}
		ans := []int{}
		assert(asteroidCollision(asteroids), ans)
	}

	{
		asteroids := []int{10, 2, -5}
		ans := []int{10}
		assert(asteroidCollision(asteroids), ans)
	}
}
