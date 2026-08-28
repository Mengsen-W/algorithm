// Package main ...
package main

import "fmt"

func lexGreaterPermutation(s string, target string) string {
	cnt := make([]int, 26)
	for _, c := range s {
		cnt[c-'a']++
	}

	var res []byte
	n := len(target)

	for i := 0; i < n; i++ {
		targetChar := int(target[i] - 'a')

		// 情况1：先尝试在当前位置放置与 target[i] 相同的字符
		if cnt[targetChar] > 0 {
			cnt[targetChar]--
			// 检查剩余字符能否构成大于 target[i+1:] 的字符串
			if canFormGreater(cnt, target, i+1) {
				res = append(res, target[i])
				continue
			}
			// 不能构成更大的字符串，回溯
			cnt[targetChar]++
		}

		// 情况2：在当前位置放置一个大于 target[i] 的字符
		for j := targetChar + 1; j < 26; j++ {
			if cnt[j] > 0 {
				cnt[j]--
				res = append(res, byte('a'+j))
				// 剩余位置按最小字典序填充
				res = append(res, getMinString(cnt)...)
				return string(res)
			}
		}

		// 无法找到可行方案, 直接返回
		return ""
	}

	return ""
}

// 检查剩余字符是否能构成大于 suffix 的字符串
func canFormGreater(cnt []int, target string, start int) bool {
	maxStr := getMaxString(cnt)
	suffix := target[start:]
	return maxStr > suffix
}

// 获取最大字典序字符串（降序排列）
func getMaxString(cnt []int) string {
	var res []byte
	for i := 25; i >= 0; i-- {
		if cnt[i] > 0 {
			for k := 0; k < cnt[i]; k++ {
				res = append(res, byte('a'+i))
			}
		}
	}
	return string(res)
}

// 获取最小字典序字符串（升序排列）
func getMinString(cnt []int) string {
	var res []byte
	for i := 0; i < 26; i++ {
		if cnt[i] > 0 {
			for k := 0; k < cnt[i]; k++ {
				res = append(res, byte('a'+i))
			}
		}
	}
	return string(res)
}

func main() {
	tests := []struct {
		s      string
		target string
		ans    string
	}{
		{"abc", "bba", "bca"},
		{"leet", "code", "eelt"},
		{"baba", "bbaa", ""},
	}

	for index, test := range tests {
		if lexGreaterPermutation(test.s, test.target) != test.ans {
			fmt.Printf("Test %d failed: expected %s, got %s\n", index, test.ans, lexGreaterPermutation(test.s, test.target))
		}
	}
}
