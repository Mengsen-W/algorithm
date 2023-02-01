/*
 * @Date: 2023-02-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-01
 * @FilePath: /algorithm/golang/2325_decode_message/decode_message.go
 */

package main

func decodeMessage(key string, message string) string {
	cur := byte('a')
	rules := map[rune]byte{}

	for _, c := range key {
		if c != ' ' && rules[c] == 0 {
			rules[c] = cur
			cur++
		}
	}

	m := []byte(message)
	for i, c := range message {
		if c != ' ' {
			m[i] = rules[c]
		}
	}

	return string(m)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		key := "the quick brown fox jumps over the lazy dog"
		message := "vkbs bs t suepuv"
		ans := "this is a secret"
		assert(decodeMessage(key, message) == ans)
	}

	{
		key := "eljuxhpwnyrdgtqkviszcfmabo"
		message := "zwx hnfx lqantp mnoeius ycgk vcnjrdb"
		ans := "the five boxing wizards jump quickly"
		assert(decodeMessage(key, message) == ans)
	}
}
