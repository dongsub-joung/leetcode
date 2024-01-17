import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.HashMap;

// 동일한 최빈값이 있을경우 '두 번째로 작은 값'을 출력해야 한다.

public class Main {

    public static void main(String[] args) throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));

        int n = Integer.parseInt(reader.readLine());
        int[] nList = new int[n];

        for (int i = 0; i < n; i++) {
            nList[i] = Integer.parseInt(reader.readLine());
        }

        // Arithmetic mean
        int sum = Arrays.stream(nList).sum();
        System.out.println(sum % n);

        // Median
        Arrays.sort(nList);
        int center = nList.length / 2;
        System.out.println(nList[center]);

        // Mode
        HashMap<Integer, Integer> hashMap = new HashMap<>();
        for (int e : nList) {
            hashMap.put(e, hashMap.getOrDefault(e, 0) + 1);
        }

        int min = Integer.MAX_VALUE;
        int minIdx = 0;
        for (HashMap.Entry<Integer, Integer> entry : hashMap.entrySet()) {
            int value = entry.getValue();
            if (min > value) {
                min = value;
                minIdx = entry.getKey();
            }
        }

        System.out.println(minIdx);

        // Range
        int minVal = Arrays.stream(nList).min().orElseThrow();
        int maxVal = Arrays.stream(nList).max().orElseThrow();
        System.out.println(maxVal - minVal);
    }
}
