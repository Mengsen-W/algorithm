/*
 * @Date: 2023-07-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-05
 * @FilePath: /algorithm/golang/2600_k_items_with_maximum_sum/k_items_with_maximum_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func kItemsWithMaximumSum(numOnes int, numZeros int, numNegOnes int, k int) int {
	if k <= numOnes {
		return k
	} else if k <= numOnes+numZeros {
		return numOnes
	} else {
		return numOnes - (k - numOnes - numZeros)
	}
}

func main() {
	testMap := []struct {
		numOnes    int
		numZeros   int
		numNegOnes int
		k          int
		ans        int
	}{
		{3, 2, 0, 2, 2},
		{3, 2, 0, 4, 3},
	}

	for _, item := range testMap {
		assert.Equal(&testing.T{}, kItemsWithMaximumSum(item.numOnes, item.numZeros, item.numNegOnes, item.k), item.ans)
	}
}
