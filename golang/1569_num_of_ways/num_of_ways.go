package main

import "fmt"

const MOD = 1000000007

type TNode struct {
	left  *TNode
	right *TNode
	size  int
	ans   int
}

func NewTNode() *TNode {
	return &TNode{
		left:  nil,
		right: nil,
		size:  1,
		ans:   0,
	}
}

type UnionFind struct {
	parent []int
	size   []int
	root   []int
	n      int
}

func NewUnionFind(n int) *UnionFind {
	parent := make([]int, n)
	size := make([]int, n)
	root := make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
		root[i] = i
		size[i] = 1
	}
	return &UnionFind{
		parent: parent,
		size:   size,
		root:   root,
		n:      n,
	}
}

func (uf *UnionFind) Findset(x int) int {
	if uf.parent[x] == x {
		return x
	}
	uf.parent[x] = uf.Findset(uf.parent[x])
	return uf.parent[x]
}

func (uf *UnionFind) Getroot(x int) int {
	return uf.root[uf.Findset(x)]
}

func (uf *UnionFind) Unite(x, y int) {
	uf.root[y] = uf.root[x]
	if uf.size[x] < uf.size[y] {
		x, y = y, x
	}
	uf.parent[y] = x
	uf.size[x] += uf.size[y]
}

func (uf *UnionFind) FindAndUnite(x, y int) bool {
	x0 := uf.Findset(x)
	y0 := uf.Findset(y)
	if x0 != y0 {
		uf.Unite(x0, y0)
		return true
	}
	return false
}

func numOfWays(nums []int) int {
	n := len(nums)
	if n == 1 {
		return 0
	}

	fac := make([]int, n)
	inv := make([]int, n)
	facInv := make([]int, n)
	fac[0] = 1
	inv[0] = 1
	facInv[0] = 1
	if n > 1 {
		fac[1] = 1
		inv[1] = 1
		facInv[1] = 1
	}
	for i := 2; i < n; i++ {
		fac[i] = int(int64(fac[i-1]) * int64(i) % MOD)
		inv[i] = int(int64(MOD-MOD/i) * int64(inv[MOD%i]) % MOD)
		facInv[i] = int(int64(facInv[i-1]) * int64(inv[i]) % MOD)
	}

	found := make(map[int]*TNode)
	uf := NewUnionFind(n)

	for i := n - 1; i >= 0; i-- {
		val := nums[i] - 1
		node := NewTNode()

		if val > 0 {
			if _, ok := found[val-1]; ok {
				lchild := uf.Getroot(val - 1)
				node.left = found[lchild]
				node.size += node.left.size
				uf.FindAndUnite(val, lchild)
			}
		}
		if val < n-1 {
			if _, ok := found[val+1]; ok {
				rchild := uf.Getroot(val + 1)
				node.right = found[rchild]
				node.size += node.right.size
				uf.FindAndUnite(val, rchild)
			}
		}

		lsize := 0
		rsize := 0
		lans := 1
		rans := 1
		if node.left != nil {
			lsize = node.left.size
			lans = node.left.ans
		}
		if node.right != nil {
			rsize = node.right.size
			rans = node.right.ans
		}

		node.ans = int(int64(fac[lsize+rsize]) * int64(facInv[lsize]) % MOD * int64(facInv[rsize]) % MOD * int64(lans) % MOD * int64(rans) % MOD)
		found[val] = node
	}

	return (found[nums[0]-1].ans - 1 + MOD) % MOD
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 1, 3}, 1},
		{[]int{3, 4, 5, 1, 2}, 5},
		{[]int{1, 2, 3}, 0},
	}

	for index, test := range tests {
		if numOfWays(test.nums) != test.ans {
			fmt.Println(index)
		}
	}
}
