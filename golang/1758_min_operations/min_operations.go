// Package main ...
package main

func minOperations(s string) int {
	cnt := 0
	for i, c := range s {
		if i%2 != int(c-'0') {
			cnt++
		}
	}
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	return min(cnt, len(s)-cnt)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	tests := []struct {
		s   string
		ans int
	}{
		{"0100", 1},
		{"10", 0},
		{"1111", 2},
	}

	for _, tt := range tests {
		assert(minOperations(tt.s) == tt.ans)
	}
}
