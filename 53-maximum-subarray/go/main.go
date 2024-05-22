package main

import ( 
    "fmt"
    "math"
)

// O(n^2) Solution
func maxSubArrayN2(nums []int) int {
    finalSum := math.MinInt16

    for i := 0; i < len(nums); i++ {
        sum := 0
        for j := i; j < len(nums); j++ {
            sum += nums[j]
            if finalSum < sum {
                finalSum = sum
            } 
        }
    }

    return finalSum
}

func maxSubArray(nums []int) int {
    if len(nums) < 1 {
        return -1
    }

    sum := 0
    finalSum := math.MinInt16
    for i := 0; i < len(nums); i++ {
        sum += nums[i]
        if finalSum < sum {
            finalSum = sum
        } 
        if sum < 0 {
            sum = 0
        }
    }

    return finalSum
}

func main() {
	// fmt.Println("Hello")

    nums := []int{-2,1,-3,4,-1,2,1,-5,4}
    fmt.Println(nums)
    fmt.Println("sum is = ", maxSubArray(nums))
}
