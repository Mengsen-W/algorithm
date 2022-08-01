/*
 * @Date: 2022-08-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-01
 * @FilePath: /algorithm/1374_generate_the_string/generate_the_string.go
 */

package main

import (
	"fmt"
	"strings"
)

func generateTheString(n int) string {
	if n%2 == 1 {
		return strings.Repeat("a", n)
	}
	return strings.Repeat("a", n-1) + "b"
}

func main() {
	fmt.Println(generateTheString(4))
	fmt.Println(generateTheString(2))
}
