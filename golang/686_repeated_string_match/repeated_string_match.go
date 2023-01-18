/*
 * @Date: 2021-12-22 00:44:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-22 02:16:55
 */

package main

func strStr(haystack, needle string) int {
	n, m := len(haystack), len(needle)
	if m == 0 {
		return 0
	}
	pi := make([]int, m)
	for i, j := 1, 0; i < m; i++ {
		for j > 0 && needle[i] != needle[j] {
			j = pi[j-1]
		}
		if needle[i] == needle[j] {
			j++
		}
		pi[i] = j
	}
	for i, j := 0, 0; i-j < n; i++ { // b 开始匹配的位置是否超过第一个叠加的 a
		for j > 0 && haystack[i%n] != needle[j] { // haystack 是循环叠加的字符串，所以取 i % n
			j = pi[j-1]
		}
		if haystack[i%n] == needle[j] {
			j++
		}
		if j == m {
			return i - m + 1
		}
	}
	return -1
}

func repeatedStringMatch(a string, b string) int {
	an, bn := len(a), len(b)
	index := strStr(a, b)
	if index == -1 {
		return -1
	}
	if an-index >= bn {
		return 1
	}
	return (bn+index-an-1)/an + 2
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	assert(repeatedStringMatch("abcd", "cdabcdab"), 3)
	assert(repeatedStringMatch("a", "aa"), 2)
	assert(repeatedStringMatch("a", "a"), 1)
	assert(repeatedStringMatch("abc", "wxyz"), -1)
}
