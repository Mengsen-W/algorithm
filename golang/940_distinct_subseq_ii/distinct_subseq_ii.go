/*
 * @Date: 2022-10-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-14
 * @FilePath: /algorithm/940_distinct_subseq_ii/distinct_subseq_ii.go
 */

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
	assert(distinctSubseqII("abc") == 7)
	assert(distinctSubseqII("aba") == 6)
	assert(distinctSubseqII("aaa") == 3)
}
