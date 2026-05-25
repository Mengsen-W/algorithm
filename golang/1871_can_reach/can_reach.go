// Package main ...
package main

func canReach(s string, minJump int, maxJump int) bool {
	n := len(s)
	f := make([]int, n)
	pre := make([]int, n)
	f[0] = 1
	// 由于我们从 i=minJump 开始动态规划，因此需要将 [0,minJump) 这部分的前缀和预处理出来
	for i := 0; i < minJump; i++ {
		pre[i] = 1
	}
	for i := minJump; i < n; i++ {
		left := i - maxJump
		right := i - minJump
		if s[i] == '0' {
			total := pre[right]
			if left > 0 {
				total -= pre[left-1]
			}
			if total != 0 {
				f[i] = 1
			} else {
				f[i] = 0
			}
		}
		pre[i] = pre[i-1] + f[i]
	}
	return f[n-1] == 1
}

func main() {
	tests := []struct {
		s       string
		minJump int
		maxJump int
		ans     bool
	}{
		{"011010", 2, 3, true},
		{"01101110", 2, 3, false},
	}

	for _, test := range tests {
		ans := canReach(test.s, test.minJump, test.maxJump)
		if ans != test.ans {
			panic("error")
		}
	}
}
