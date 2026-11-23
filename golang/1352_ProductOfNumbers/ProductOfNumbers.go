// Package main ...
package main

type ProductOfNumbers struct {
	nums       []int
	zeroCount  []int
	prevNonOne []int
	n          int
}

func Constructor() ProductOfNumbers {
	return ProductOfNumbers{
		nums:       []int{},
		zeroCount:  []int{},
		prevNonOne: []int{},
		n:          0,
	}
}

func (this *ProductOfNumbers) Add(num int) {
	this.n++
	this.nums = append(this.nums, num)

	this.prevNonOne = append(this.prevNonOne, -1)
	if this.n > 1 {
		if this.nums[this.n-2] != 1 && this.nums[this.n-2] != 0 {
			this.prevNonOne[this.n-1] = this.n - 2
		} else {
			this.prevNonOne[this.n-1] = this.prevNonOne[this.n-2]
		}
	}

	zeroVal := 0
	if num == 0 {
		zeroVal = 1
	}
	this.zeroCount = append(this.zeroCount, zeroVal)
	if this.n > 1 {
		this.zeroCount[this.n-1] += this.zeroCount[this.n-2]
	}
}

func (this *ProductOfNumbers) GetProduct(k int) int {
	totalZeros := this.zeroCount[this.n-1]
	if this.n-k >= 1 {
		totalZeros -= this.zeroCount[this.n-1-k]
	}
	if totalZeros > 0 {
		return 0
	}

	ans := 1
	for i := this.n - 1; i >= this.n-k; {
		ans *= this.nums[i]
		i = this.prevNonOne[i]
	}
	return ans
}

func main() {
	productOfNumbers := ProductOfNumbers{}
	productOfNumbers.Add(3)        // [3]
	productOfNumbers.Add(0)        // [3,0]
	productOfNumbers.Add(2)        // [3,0,2]
	productOfNumbers.Add(5)        // [3,0,2,5]
	productOfNumbers.Add(4)        // [3,0,2,5,4]
	productOfNumbers.GetProduct(2) // 返回 20 。最后 2 个数字的乘积是 5 * 4 = 20
	productOfNumbers.GetProduct(3) // 返回 40 。最后 3 个数字的乘积是 2 * 5 * 4 = 40
	productOfNumbers.GetProduct(4) // 返回  0 。最后 4 个数字的乘积是 0 * 2 * 5 * 4 = 0
	productOfNumbers.Add(8)        // [3,0,2,5,4,8]
	productOfNumbers.GetProduct(2) // 返回 32 。最后 2 个数字的乘积是 4 * 8 = 32
}
