package main

import (
	"fmt"
	"slices"
)

func permute(nums []int) [][]int {
	var ans [][]int
	recursive_permute(0, nums, ans)

	return ans
}

func swap(i int, j int, nums []int) {
	temp := nums[i]
	nums[i] = nums[j]
	nums[j] = temp
}

func recursive_permute(index int, nums []int, ans [][]int) {
	fmt.Println("Ans = ", ans)
	if index >= len(nums) {
		fmt.Println("Permutation = ", nums)

		permutation := slices.Clone(nums)
		// Pass By Reference Occuring Here
		// Don't Know How to Pass 'nums' By Value
		ans = append(ans, permutation)
		fmt.Println("Ans = ", ans)

		return
	}

	for i := index; i < len(nums); i++ {
		swap(index, i, nums)
		fmt.Println("Ans = ", ans)
		recursive_permute(index+1, nums, ans)
		swap(i, index, nums)
	}
}

func main() {
	nums := []int{1, 2, 3}
	fmt.Println("Nums are = ", nums)
	fmt.Println("Permutations are = ", permute(nums))
}
