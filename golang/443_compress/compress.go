/*
 * @Date: 2021-08-21 14:21:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-21 14:57:10
 */

package main

func compress(chars []byte) int {
	write, left := 0, 0
	for read, ch := range chars {
		if read == len(chars)-1 || ch != chars[read+1] {
			chars[write] = ch
			write++
			num := read - left + 1
			if num > 1 {
				anchor := write
				for ; num > 0; num /= 10 {
					chars[write] = '0' + byte(num%10)
					write++
				}
				s := chars[anchor:write]
				for i, n := 0, len(s); i < n/2; i++ {
					s[i], s[n-1-i] = s[n-1-i], s[i]
				}
			}
			left = read + 1
		}
	}
	return write
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		chars := []byte{'a', 'a', 'b', 'b', 'c', 'c', 'c'}
		assert(compress(chars) == 6)
	}
	{
		chars := []byte{'a'}
		assert(compress(chars) == 1)
	}
	{
		chars := []byte{'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'}
		assert(compress(chars) == 4)
	}
}
