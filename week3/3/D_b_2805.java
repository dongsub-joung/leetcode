// https://www.acmicpc.net/problem/2805
// https://st-lab.tistory.com/270


/*
 * [upper bound 방식]
 * int[] tree : 입력받은 나무들의 높이가 저장되어있는 배열(정렬할 필요 없음)
 * min : 하한선 변수로 초기값은 0
 * max : tree 에 저장 된 나무들 중 가장 큰 나무의 높이
 */
 
while(min < max) {
 
	// 중간 값(=자르는 위치)을 구한다.
	int mid = (min + max) / 2;
	long sum = 0;
 
 
	for(int treeHeight : tree) {
				
		/*
		 *  tree의 잘린 길이 = tree의 높이 - 자르는 위치(mid)
		 *  tree의 잘린 길의의 합을 구한다.
		 *  이 때 자르는 위치가 주어진 나무의 높이보다 클 수 있기 때문에
		 *  0 이하인 경우(=음수)에는 합산을 하지 않고 양수만 합산하도록 해야한다.
		 */
		if(treeHeight - mid > 0) { 
			sum += (treeHeight - mid);
		}
	}
			
 
	/*
	 * 자른 나무 길의의 합이 M보다 작다는 것은
	 * 자르는 위치(높이)가 높다는 의미이므로 높이를 낮춰야 한다.
	 * 즉, 상한(max)를 줄여야 한다.
	 */
	if(sum < M) {
		max = mid;
	}
			
	/*
	 * 자른 나무 길이의 합이 M보다 크다는 것은
	 * 자르는 위치(높이)가 낮다는 의미이므로 높이를 높여야 한다.
	 * 또한, 같을 경우에는 최대한 적게 자르기 위해 자르는 높이를
	 * 높여야 하므로 하한(min)을 올려야 한다.
	 */
	else {
		min = mid + 1;
	}
}
 
// 출력할 결과 값 : min - 1