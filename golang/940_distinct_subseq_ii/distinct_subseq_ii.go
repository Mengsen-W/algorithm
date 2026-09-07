// Package main ...
package main

func distinctSubseqII(s string) (ans int) {
	const mod int = 1e9 + 7
	last := make([]int, 26)
	for i := range last {
		last[i] = -1
	}
	n := len(s)
	f := make([]int, n)
	for i := range f {
		f[i] = 1
	}
	for i, c := range s {
		for _, j := range last {
			if j != -1 {
				f[i] = (f[i] + f[j]) % mod
			}
		}
		last[c-'a'] = i
	}
	for _, i := range last {
		if i != -1 {
			ans = (ans + f[i]) % mod
		}
	}
	return
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
		{"abc", 7},
		{"aba", 6},
		{"aaa", 3},
	}

	for _, test := range tests {
		assert(distinctSubseqII(test.s) == test.ans)
	}
}
