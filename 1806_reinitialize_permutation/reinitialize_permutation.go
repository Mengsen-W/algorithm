/*
 * @Date: 2023-01-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-09
 * @FilePath: /algorithm/1806_reinitialize_permutation/reinitialize_permutation.go
 */

package main

func reinitializePermutation(n int) int {
	if n == 2 {
		return 1
	}
	step := 1
	pow2 := 2
	for pow2 != 1 {
		step++
		pow2 = pow2 * 2 % (n - 1)
	}
	return step
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 2
		ans := 1
		assert(reinitializePermutation(n) == ans)
	}

	{
		n := 4
		ans := 2
		assert(reinitializePermutation(n) == ans)
	}

	{
		n := 6
		ans := 4
		assert(reinitializePermutation(n) == ans)
	}
}
