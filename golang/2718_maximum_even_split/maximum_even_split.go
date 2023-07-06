/*
 * @Date: 2023-07-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-06
 * @FilePath: /algorithm/golang/2718_maximum_even_split/maximum_even_split.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumEvenSplit(finalSum int64) []int64 {
	var res []int64
	if finalSum%2 > 0 {
		return res
	}
	for i := int64(2); i <= finalSum; i += 2 {
		res = append(res, i)
		finalSum -= i
	}
	res[len(res)-1] += finalSum
	return res
}

func main() {
	testMap := []struct {
		finalSum int64
		ans      []int64
	}{
		{12, []int64{2, 4, 6}},
		{7, []int64{}},
		{28, []int64{6, 8, 2, 12}},
	}

	for _, item := range testMap {
		assert.Equal(&testing.T{}, maximumEvenSplit(item.finalSum), item.ans)
	}
}
