package main

import "fmt"

func subsets(nums []int) [][]int {
    var allSubsets [][]int 
    subset :=  []int {}
    getSubset(nums, subset, &allSubsets, 0)

    return allSubsets
}

func getSubset(nums []int, subset []int, allSubsets *[][]int, index int)  {
    tempSubset := make([]int, len(subset))
    copy(tempSubset, subset)
    *allSubsets = append(*allSubsets, tempSubset)

    for i := index; i < len(nums); i++ {
        subset = append(subset, nums[i])
        getSubset(nums, subset, allSubsets, i + 1)
        subset = subset[:len(subset)-1]
    }
}

func main() {
    nums := []int {1,2,3}
    fmt.Println("Nums are = ", nums)
	fmt.Println("Subsets are = ", subsets(nums))
}
