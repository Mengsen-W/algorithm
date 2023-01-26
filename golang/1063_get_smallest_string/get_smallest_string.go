/*
 * @Date: 2023-01-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-26
 * @FilePath: /algorithm/golang/1063_get_smallest_string/get_smallest_string.go
 */

package main

func getSmallestString(n, k int) string {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	s := []byte{}
	for i := 1; i <= n; i++ {
		lower := max(1, k-(n-i)*26)
		k -= lower
		s = append(s, 'a'+byte(lower)-1)
	}
	return string(s)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		n := 3
		k := 27
		ans := "aay"
		assert(getSmallestString(n, k) == ans)
	}

	{
		n := 5
		k := 73
		ans := "aaszz"
		assert(getSmallestString(n, k) == ans)
	}
}
