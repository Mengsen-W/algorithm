/*
 * @Date: 2023-05-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-14
 * @FilePath: /algorithm/golang/1054_rearrange_barcodes/rearrange_barcodes.go
 */

// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func rearrangeBarcodes(barcodes []int) []int {
	if len(barcodes) < 2 {
		return barcodes
	}

	counts := make(map[int]int)
	maxCount := 0
	for _, b := range barcodes {
		counts[b] = counts[b] + 1
		if counts[b] > maxCount {
			maxCount = counts[b]
		}
	}

	evenIndex := 0
	oddIndex := 1
	halfLength := len(barcodes) / 2
	res := make([]int, len(barcodes))
	for x, count := range counts {
		for count > 0 && count <= halfLength && oddIndex < len(barcodes) {
			res[oddIndex] = x
			count--
			oddIndex += 2
		}
		for count > 0 {
			res[evenIndex] = x
			count--
			evenIndex += 2
		}
	}
	return res
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic(fmt.Sprintf("%+v not equal %+v", a, b))
		}
	}

	{
		barcodes := []int{1, 1, 1, 2, 2, 2}
		ans := []int{2, 1, 2, 1, 2, 1}
		assert(rearrangeBarcodes(barcodes), ans)
	}

	{
		barcodes := []int{1, 1, 1, 1, 2, 2, 3, 3}
		ans := []int{2, 1, 2, 1, 3, 1, 3, 1}
		assert(rearrangeBarcodes(barcodes), ans)
	}
}
