package main

import (
	"fmt"
	"slices"
)

func permute(nums []int) [][]int {
	var ans [][]int
	recursive_permute(0, nums, &ans)

	return ans
}

func swap(i int, j int, nums []int) {
	temp := nums[i]
	nums[i] = nums[j]
	nums[j] = temp
}

func recursive_permute(index int, nums []int, ans *[][]int) {
	if index >= len(nums) {
		permutation := slices.Clone(nums)
		*ans = append(*ans, permutation)

		return
	}

	for i := index; i < len(nums); i++ {
		swap(index, i, nums)
		recursive_permute(index + 1, nums, ans)
		swap(i, index, nums)
	}
}

func main() {
	nums := []int{1, 2, 3}
	fmt.Println("Nums are = ", nums)
	fmt.Println("Permutations are = ", permute(nums))
}
