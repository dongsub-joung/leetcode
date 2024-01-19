// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/
// https://ifuwanna.tistory.com/347s

public int[] twoSum(int[] numbers, int target) {

    //2. Two Pointers
    int len = numbers.length;
    int start = 0;
    int end = len-1;

    while(start < end){
        int sum = numbers[start] + numbers[end];
        if(target == sum){
            return new int[]{start+1,end+1};
        }else if(target < sum){ // sum을 줄여야하니 end--
            end--;
        }else{ // target < sum){ //sum을 늘려야하니 start++
            start++;
        }
    }
    return null;
}
