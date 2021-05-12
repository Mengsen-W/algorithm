/*
 * @Date: 2021-05-12 08:57:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-12 09:32:22
 */

package main

import "fmt"

func xorQueries(arr []int, queries [][]int) []int {
	xors := make([]int, len(arr)+1)
	for i, v := range arr {
		xors[i+1] = xors[i] ^ v
	}
	ans := make([]int, len(queries))
	for i, q := range queries {
		ans[i] = xors[q[0]] ^ xors[q[1]+1]
	}
	return ans
}

func main() {
	{
		arr := [...]int{1, 3, 4, 8}
		queries := [][]int{{0, 1}, {1, 2}, {0, 3}, {3, 3}}
		fmt.Printf("%d\n", xorQueries(arr[:], queries[:][:]))
	}
	{
		arr := [...]int{4, 8, 2, 10}
		queries := [][]int{{2, 3}, {1, 3}, {0, 0}, {0, 3}}
		fmt.Printf("%d\n", xorQueries(arr[:], queries[:][:]))
	}
}
