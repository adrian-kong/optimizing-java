package dev;

/**
 * This snippet is taken from
 * <p>
 * Optimizing Java
 * by Chris Newland, Benjamin Evans, James Gough
 * <p>
 * (page 39)
 * <p>
 * all credits to author.
 * <p>
 * Intends to show intuitive vs actual performance is drastically different,
 * JVM performance benchmarking memory cache,
 * updating every element in array vs skipping
 */
public class Cache {
    private final int ARR_SIZE = 2 * 1024 * 1024;
    private final int[] testData = new int[ARR_SIZE];

    public void run() {
        System.out.println("START:" + System.currentTimeMillis());
        for (int i = 0; i < 15_000L; i++) {
            touchEveryLine();
//            touchEveryItem();
        }
        System.out.println("WARMUP FINISHED:" + System.currentTimeMillis());
        System.out.println("item line");
        for (int i = 0; i < 100; i++) {
            long t0 = System.nanoTime();
            touchEveryLine();
            long t1 = System.nanoTime();
            System.out.println(t1 - t0);
//            touchEveryItem();
//            long t2 = System.nanoTime();
//            long elLine = t1 - t0;
//            long elItem = t2 - t1;
//            double diff = elItem - elLine;
//            System.out.println(elItem + " " + elLine + " " + (100 * diff / elLine));
        }
    }

    public void touchEveryLine() {
        // Interestingly, incrementing 'i' by values:
        // < 16 is significantly slower than iterating element by element
        // {1, 16} is approximately equivalent.
        // > 16 is significantly faster than iterating element by element
        for (int i = 0; i < testData.length; i += 5) {
            testData[i]++;
        }
    }

    public void touchEveryItem() {
        for (int i = 0; i < testData.length; i++) {
            testData[i]++;
        }
    }


}