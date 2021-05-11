/*
 * @Date: 2021-05-11 08:31:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-11 08:54:50
 */

package main

import "fmt"

func decode(encoded []int) []int {
	n := len(encoded)
	total := 0
	for i := 1; i <= n+1; i++ {
		total ^= i
	}
	odd := 0
	for i := 1; i < n; i += 2 {
		odd ^= encoded[i]
	}
	perm := make([]int, n+1)
	perm[0] = total ^ odd
	for i, v := range encoded {
		perm[i+1] = perm[i] ^ v
	}
	return perm
}

func main() {
	{
		encoded := [...]int{3, 1}
		fmt.Printf("%d\n", decode(encoded[:]))
	}
	{
		encoded := [...]int{6, 5, 4, 6}
		fmt.Printf("%d\n", decode(encoded[:]))
	}
}
