/*
 * @Date: 2021-06-18 08:01:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-18 08:14:00
 */

package main

import (
	"math"
	"math/bits"
	"strconv"
)

func smallestGoodBase(n string) string {
	nVal, _ := strconv.Atoi(n)
	mMax := bits.Len(uint(nVal)) - 1
	for m := mMax; m > 1; m-- {
		k := int(math.Pow(float64(nVal), 1/float64(m)))
		mul, sum := 1, 1
		for i := 0; i < m; i++ {
			mul *= k
			sum += mul
		}
		if sum == nVal {
			return strconv.Itoa(k)
		}
	}
	return strconv.Itoa(nVal - 1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(smallestGoodBase("13") == "3")
	assert(smallestGoodBase("4681") == "8")
	assert(smallestGoodBase("1000000000000000000") == "999999999999999999")
}
