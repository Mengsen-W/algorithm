// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const mx = 101

var notPrime = [mx]bool{true, true}

func init() {
	for i := 2; i*i < mx; i++ {
		if !notPrime[i] {
			for j := i * i; j < mx; j += i {
				notPrime[j] = true // j 是质数 i 的倍数
			}
		}
	}
}

func maximumPrimeDifference(nums []int) int {
	i := 0
	for notPrime[nums[i]] {
		i++
	}
	j := len(nums) - 1
	for notPrime[nums[j]] {
		j--
	}
	return j - i
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{4, 2, 9, 5, 3}, 3},
		{[]int{4, 8, 2, 8}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumPrimeDifference(test.nums), index)
	}
}
