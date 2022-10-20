/*
 * @Date: 2022-10-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-20
 * @FilePath: /algorithm/779_kth_grammar/kth_grammar.go
 */

package main

func kthGrammar(n, k int) (ans int) {
	// return bits.OnesCount(uint(k-1)) & 1
	for k--; k > 0; k &= k - 1 {
		ans ^= 1
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(kthGrammar(1, 1) == 0)
	assert(kthGrammar(2, 1) == 0)
	assert(kthGrammar(2, 2) == 1)
}
