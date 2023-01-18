/*
 * @Date: 2021-05-07 09:54:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-07 10:16:08
 */

package main

import "fmt"

func xor_operation(n int, start int) int {
	ans := 0

	for i := 0; i < n; i++ {
		ans ^= start + i*2
	}
	return ans
}

func main() {
	if xor_operation(5, 0) != 8 {
		fmt.Printf("Not Passed %d", xor_operation(5, 0))
	}
	if xor_operation(4, 3) != 8 {
		fmt.Printf("Not Passed %d", xor_operation(4, 3))
	}
	if xor_operation(1, 7) != 7 {
		fmt.Printf("Not Passed %d", xor_operation(1, 7))
	}
	if xor_operation(10, 5) != 2 {
		fmt.Printf("Not Passed %d", xor_operation(10, 5))
	}
}
