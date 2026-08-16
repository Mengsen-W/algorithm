// Package main ...
package main

func stoneGameIX(stones []int) bool {
	cnt0, cnt1, cnt2 := 0, 0, 0
	for _, val := range stones {
		val %= 3
		if val == 0 {
			cnt0++
		} else if val == 1 {
			cnt1++
		} else {
			cnt2++
		}
	}
	if cnt0%2 == 0 {
		return cnt1 >= 1 && cnt2 >= 1
	}
	return cnt1-cnt2 > 2 || cnt2-cnt1 > 2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	tests := []struct {
		stones []int
		ans    bool
	}{
		{[]int{2, 1}, true},
		{[]int{2}, false},
		{[]int{5, 1, 2, 4, 3}, false},
	}

	for _, test := range tests {
		assert(stoneGameIX(test.stones) == test.ans)
	}
}
