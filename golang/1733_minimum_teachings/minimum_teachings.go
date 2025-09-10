// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumTeachings(n int, languages [][]int, friendships [][]int) int {
	cncon := make(map[int]bool)
	for _, friendship := range friendships {
		mp := make(map[int]bool)
		conm := false
		for _, lan := range languages[friendship[0]-1] {
			mp[lan] = true
		}
		for _, lan := range languages[friendship[1]-1] {
			if mp[lan] {
				conm = true
				break
			}
		}
		if !conm {
			cncon[friendship[0]-1] = true
			cncon[friendship[1]-1] = true
		}
	}

	maxCnt := 0
	cnt := make([]int, n+1)
	for person := range cncon {
		for _, lan := range languages[person] {
			cnt[lan]++
			maxCnt = max(maxCnt, cnt[lan])
		}
	}

	return len(cncon) - maxCnt
}

func main() {
	tests := []struct {
		n           int
		languages   [][]int
		friendships [][]int
		expected    int
	}{
		{2, [][]int{{1}, {2}, {1, 2}}, [][]int{{1, 2}, {1, 3}, {2, 3}}, 1},
		{3, [][]int{{2}, {1, 3}, {1, 2}, {3}}, [][]int{{1, 4}, {1, 2}, {3, 4}, {2, 3}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, minimumTeachings(test.n, test.languages, test.friendships), index)
	}
}
