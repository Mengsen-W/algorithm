/*
 * @Date: 2022-09-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-15
 * @FilePath: /algorithm/672_flip_lights/flip_lights.go
 */

package main

func flipLights(n int, presses int) int {
	bool2int := func(b bool) int {
		if b {
			return 1
		} else {
			return 0
		}
	}
	if presses > 2 && n > 2 {
		return 8
	}
	if n < 3 {
		return 1 + bool2int(presses > 0)*n + bool2int(presses > 1 && n > 1)
	} else {
		return 1 + 3*presses
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(flipLights(1, 1) == 2)
	assert(flipLights(2, 1) == 3)
	assert(flipLights(3, 1) == 4)
}
