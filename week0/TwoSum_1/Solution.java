 class Solution {
    public int[] twoSum(int[] nums, int target) {
        for(int i=0; i<nums.length; i++){
            for (int j=0; j< nums.length; j++){
                if (nums[i] + nums[j] == target){
                    return new int[]{i, j};
                }
            }
        }
        return new int[]{0,0};
    }

    public static void main(String[] args) {
        Solution solution= new Solution();
        var result= solution.twoSum(new int[]{2,7,11,15}, 9);
        for (var i : result){
            System.out.println(i);
        }
//        Output: [0,1]
    }
}
