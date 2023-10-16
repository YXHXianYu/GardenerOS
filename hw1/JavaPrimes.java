import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class JavaPrimes {

    private static boolean check(int x) {
        for(int i = 2, lim = (int) Math.sqrt((double)x); i <= lim; i++) {
            if(x % i == 0) {
                return false;
            }
        }
        return true;
    }

    private static class MyThread extends Thread {
        int x;

        public MyThread(int x) {
            this.x = x;
        }

        public void run() {
            if(JavaPrimes.check(x)) {
                System.out.print(x + " ");
            }
        }
    }

    public static void main(String[] args) {

        Scanner scanner = new Scanner(System.in);

        int n = scanner.nextInt();

        List<Thread> threads = new ArrayList<Thread>();
        
        for(int i = 2; i <= n; i++) {
            Thread thread = new MyThread(i);
            thread.start();
            threads.add(thread);
        }

        threads.forEach(thread -> {
            try {
                thread.join();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        });

        scanner.close();

    }
}
