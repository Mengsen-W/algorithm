/*
 * @Date: 2023-04-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-12
 * @FilePath: /algorithm/golang/1147_longest_decomposition/longest_decomposition.go
 */

// Package main ...
package main

func longestDecomposition(text string) (ans int) {
	n := len(text)
	base := 131
	h := make([]int, n+10)
	p := make([]int, n+10)
	p[0] = 1
	for i, c := range text {
		t := int(c-'a') + 1
		p[i+1] = p[i] * base
		h[i+1] = h[i]*base + t
	}
	get := func(l, r int) int {
		return h[r] - h[l-1]*p[r-l+1]
	}

	for i, j := 0, n-1; i <= j; {
		ok := false
		for k := 1; i+k-1 < j-k+1; k++ {
			if get(i+1, i+k) == get(j-k+2, j+1) {
				ans += 2
				i += k
				j -= k
				ok = true
				break
			}
		}
		if !ok {
			ans++
			break
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

	{
		text := "ghiabcdefhelloadamhelloabcdefghi"
		ans := 7
		assert(longestDecomposition(text) == ans)
	}

	{
		text := "merchant"
		ans := 1
		assert(longestDecomposition(text) == ans)
	}

	{
		text := "antaprezatepzapreanta"
		ans := 11
		assert(longestDecomposition(text) == ans)
	}
}
