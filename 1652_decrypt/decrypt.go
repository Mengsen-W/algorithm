/*
 * @Date: 2022-09-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-24
 * @FilePath: /algorithm/1652_decrypt/decrypt.go
 */

package main

import "reflect"

func decrypt(code []int, k int) []int {
	n := len(code)
	ans := make([]int, n)
	if k == 0 {
		return ans
	}
	code = append(code, code...)
	l, r := 1, k
	if k < 0 {
		l, r = n+k, n-1
	}
	sum := 0
	for _, v := range code[l : r+1] {
		sum += v
	}
	for i := range ans {
		ans[i] = sum
		sum -= code[l]
		sum += code[r+1]
		l, r = l+1, r+1
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		code := []int{5, 7, 1, 4}
		k := 3
		ans := []int{12, 10, 16, 13}
		assert(decrypt(code, k), ans)
	}

	{
		code := []int{1, 2, 3, 4}
		k := 0
		ans := []int{0, 0, 0, 0}
		assert(decrypt(code, k), ans)
	}

	{
		code := []int{2, 4, 9, 3}
		k := -2
		ans := []int{12, 5, 6, 13}
		assert(decrypt(code, k), ans)
	}
}
