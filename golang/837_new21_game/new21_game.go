// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func new21Game(n, k, maxPts int) float64 {
	f := make([]float64, n+1)
	s := 0.0
	for i := n; i >= 0; i-- {
		if i >= k {
			f[i] = 1 // 初始值
		} else {
			f[i] = s / float64(maxPts)
		}
		// 当前循环计算的是 f[i+1] + ... + f[i+maxPts]
		// 下个循环计算的是 f[i] + ... + f[i+maxPts-1]，多了 f[i]，少了 f[i+maxPts]
		s += f[i]
		if i+maxPts <= n {
			s -= f[i+maxPts]
		}
	}
	return f[0]
}

func main() {
	tests := []struct {
		n, k, maxPts int
		ans          float64
	}{
		{10, 1, 10, 1.0000},
		{6, 1, 10, 0.60000},
		{21, 17, 10, 0.73278},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, new21Game(test.n, test.k, test.maxPts), index)
	}
}
