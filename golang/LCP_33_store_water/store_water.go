/*
 * @Date: 2023-05-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-21
 * @FilePath: /algorithm/golang/LCP_33_store_water/store_water.go
 */

// Package main ...
package main

import "math"

func storeWater(bucket []int, vat []int) int {
	n := len(bucket)
	maxk := 0
	for _, v := range vat {
		if v > maxk {
			maxk = v
		}
	}
	if maxk == 0 {
		return 0
	}
	res := math.MaxInt32
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}

	min := func(x, y int) int {
		if x < y {
			return x
		}
		return y
	}
	for k := 1; k <= maxk && k < res; k++ {
		t := 0
		for i := 0; i < n; i++ {
			t += max(0, (vat[i]+k-1)/k-bucket[i])
		}
		res = min(res, t+k)
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		bucket := []int{1, 3}
		vat := []int{6, 8}
		ans := 4
		assert(storeWater(bucket, vat) == ans)
	}

	{
		bucket := []int{9, 0, 1}
		vat := []int{0, 2, 2}
		ans := 3
		assert(storeWater(bucket, vat) == ans)
	}
}
