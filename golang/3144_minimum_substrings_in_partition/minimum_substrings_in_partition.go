// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSubstringsInPartition(s string) int {
	const inf = 0x3f3f3f3f
	n := len(s)
	d := make([]int, n+1)
	for i := range d {
		d[i] = inf
	}
	d[0] = 0

	for i := 1; i <= n; i++ {
		maxCnt := 0
		occCnt := make(map[byte]int)
		for j := i; j >= 1; j-- {
			occCnt[s[j-1]]++
			if occCnt[s[j-1]] > maxCnt {
				maxCnt = occCnt[s[j-1]]
			}
			if maxCnt*len(occCnt) == (i-j+1) && d[j-1] != inf {
				if d[i] > d[j-1]+1 {
					d[i] = d[j-1] + 1
				}
			}
		}
	}
	return d[n]
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"fabccddg", 3},
		{"abababaccddb", 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSubstringsInPartition(test.s), index)
	}
}
