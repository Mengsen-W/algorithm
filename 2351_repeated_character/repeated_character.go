/*
 * @Date: 2023-01-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-01
 * @FilePath: /algorithm/2351_repeated_character/repeated_character.go
 */

package main

func repeatedCharacter(s string) byte {
	seen := 0
	for _, c := range s {
		if seen>>(c-'a')&1 > 0 {
			return byte(c)
		}
		seen |= 1 << (c - 'a')
	}
	return 0 // impossible
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "abccbaacz"
		ans := byte('c')
		assert(repeatedCharacter(s) == ans)
	}

	{
		s := "abcdd"
		ans := byte('d')
		assert(repeatedCharacter(s) == ans)
	}
}
