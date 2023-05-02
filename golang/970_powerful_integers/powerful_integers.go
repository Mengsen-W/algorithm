/*
 * @Date: 2023-05-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-02
 * @FilePath: /algorithm/golang/970_powerful_integers/powerful_integers.go
 */

// Package main ...
package main

import (
	"reflect"
)

func powerfulIntegers(x int, y int, bound int) []int {
	res := make(map[int]bool)
	value1 := 1
	for i := 0; i < 21; i++ {
		value2 := 1
		for j := 0; j < 21; j++ {
			value := value1 + value2
			if value <= bound {
				res[value] = true
			} else {
				break
			}
			value2 *= y
		}
		if value1 > bound {
			break
		}
		value1 *= x
	}
	var result []int
	for k := range res {
		result = append(result, k)
	}
	return result
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		x := 2
		y := 3
		bound := 10
		ans := []int{2, 3, 4, 5, 7, 9, 10}
		assert(powerfulIntegers(x, y, bound), ans)
	}

	{
		x := 3
		y := 5
		bound := 15
		ans := []int{2, 4, 6, 8, 10, 14}
		assert(powerfulIntegers(x, y, bound), ans)
	}
}
