// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumBeauty(flowers []int, newFlowers int64, target int, full int, partial int) int64 {
	n := len(flowers)
	for i := 0; i < n; i++ {
		if flowers[i] > target {
			flowers[i] = target
		}
	}
	sort.Sort(sort.Reverse(sort.IntSlice(flowers)))
	var sum int64
	for _, flower := range flowers {
		sum += int64(flower)
	}
	var ans int64
	if int64(target)*int64(n)-sum <= newFlowers {
		ans = int64(full) * int64(n)
	}
	pre, ptr := int64(0), 0
	for i := 0; i < n; i++ {
		if i != 0 {
			pre += int64(flowers[i-1])
		}
		if flowers[i] == target {
			continue
		}
		rest := newFlowers - (int64(target)*int64(i) - pre)
		if rest < 0 {
			break
		}
		for !(ptr >= i && int64(flowers[ptr])*int64(n-ptr)-sum <= rest) {
			sum -= int64(flowers[ptr])
			ptr++
		}
		rest -= int64(flowers[ptr])*int64(n-ptr) - sum
		ans = max(ans, int64(full)*int64(i)+int64(partial)*min(int64(flowers[ptr])+rest/int64(n-ptr), int64(target)-1))
	}

	return ans
}

func main() {
	tests := []struct {
		flowers    []int
		newFlowers int64
		target     int
		full       int
		partial    int
		ans        int64
	}{
		{[]int{1, 3, 1, 1}, 7, 6, 12, 1, 14},
		{[]int{2, 4, 5, 3}, 10, 5, 2, 6, 30},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumBeauty(test.flowers, test.newFlowers, test.target, test.full, test.partial), index)
	}
}
