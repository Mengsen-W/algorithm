// Package main ...
package main

func winnerSquareGame(n int) bool {
	f := make([]bool, n+1)
	for i := 1; i <= n; i++ {
		k := 1
		for k*k <= i {
			if !f[i-k*k] {
				f[i] = true
				break
			}
			k++
		}
	}

	return f[n]
}

func main() {
	tests := []struct {
		n   int
		ans bool
	}{
		{1, true},
		{2, false},
		{7, false},
		{17, false},
	}

	for _, test := range tests {
		if ret := winnerSquareGame(test.n); ret != test.ans {
			panic("error")
		}
	}
}
