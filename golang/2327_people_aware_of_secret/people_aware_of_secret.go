// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const MOD = 1000000007

type Pair struct {
	day   int
	count int
}

func peopleAwareOfSecret(n int, delay int, forget int) int {
	know := make([]Pair, 0)
	share := make([]Pair, 0)
	know = append(know, Pair{1, 1})
	knowCnt, shareCnt := 1, 0

	for i := 2; i <= n; i++ {
		if len(know) > 0 && know[0].day == i-delay {
			first := know[0]
			know = know[1:]
			knowCnt = (knowCnt - first.count + MOD) % MOD
			shareCnt = (shareCnt + first.count) % MOD
			share = append(share, first)
		}
		if len(share) > 0 && share[0].day == i-forget {
			first := share[0]
			share = share[1:]
			shareCnt = (shareCnt - first.count + MOD) % MOD
		}
		if len(share) > 0 {
			knowCnt = (knowCnt + shareCnt) % MOD
			know = append(know, Pair{i, shareCnt})
		}
	}
	return (knowCnt + shareCnt) % MOD
}

func main() {
	tests := []struct {
		n      int
		delay  int
		forget int
		ans    int
	}{
		{6, 2, 4, 5},
		{4, 1, 3, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, peopleAwareOfSecret(test.n, test.delay, test.forget), index)
	}
}
