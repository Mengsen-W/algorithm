/*
 * @Date: 2022-12-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-08
 * @FilePath: /algorithm/1812_square_is_white/square_is_white.go
 */

package main

func squareIsWhite(coordinates string) bool {
	return ((coordinates[0]-'a'+1)+(coordinates[1]-'0'))%2 == 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(!squareIsWhite("a1"))
	assert(squareIsWhite("h3"))
	assert(!squareIsWhite("c7"))
}
