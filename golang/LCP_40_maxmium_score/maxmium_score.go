// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxmiumScore(cards []int, cnt int) int {
	val := make([]int, 1005)
	for _, card := range cards {
		val[card]++
	}
	ans := 0
	tmp := 0
	ed := -1
	odd, even := -1, -1
	for i := 1000; i >= 1; i-- {
		if val[i] == 0 {
			continue
		}
		if val[i] > cnt {
			tmp += cnt * i
			cnt = 0
		} else {
			tmp += val[i] * i
			cnt -= val[i]
			val[i] = 0
		}
		if i%2 == 1 {
			odd = i
		} else {
			even = i
		}
		if cnt == 0 {
			if val[i] > 0 {
				ed = i
			} else {
				ed = i - 1
			}
			break
		}
	}

	if tmp%2 == 0 {
		return tmp
	}
	for i := ed; i >= 1; i-- {
		if val[i] == 0 {
			continue
		}
		if i%2 == 1 {
			if even != -1 {
				ans = max(ans, tmp-even+i)
			}
		} else {
			if odd != -1 {
				ans = max(ans, tmp-odd+i)
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		cards []int
		cnt   int
		ans   int
	}{
		{[]int{1, 2, 8, 9}, 3, 18},
		{[]int{3, 3, 1}, 1, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxmiumScore(test.cards, test.cnt), index)
	}
}
