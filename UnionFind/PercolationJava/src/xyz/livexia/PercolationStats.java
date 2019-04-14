import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.StdRandom;
import edu.princeton.cs.algs4.StdStats;

public class PercolationStats {
    private double mean;
    private double stddev;
    private int t;
    private static final double CONFIDENCE_95 = 1.96;
    public PercolationStats(int n, int trials) {
        if (n <= 0 || trials <= 0) {
            throw new IllegalArgumentException();
        }
        double[] x = new double[trials];
        t = trials;


        for (int i = 0; i < trials; i++) {
            Percolation p = new Percolation(n);
            while (!p.percolates()) {
                int row = StdRandom.uniform(1,n+1);
                int col = StdRandom.uniform(1,n+1);
                p.open(row, col);
            }
            x[i] = (double) p.numberOfOpenSites() / (n * n);
        }

        mean = StdStats.mean(x);
        stddev = StdStats.stddev(x);
    }

    public double mean() {
        return mean;
    }

    public double stddev() {
        return stddev;
    }

    public double confidenceLo() {
        return mean - (CONFIDENCE_95 * stddev) / Math.sqrt(t);
    }

    public double confidenceHi() {
        return mean + (CONFIDENCE_95 * stddev) / Math.sqrt(t);
    }


    public static void main(String[] args) {
        int n = Integer.parseInt(args[0]);
        int trials = Integer.parseInt(args[1]);
        PercolationStats PS = new PercolationStats(n, trials);
        StdOut.println("mean = " + PS.mean());
        StdOut.println("stddev = " + PS.stddev());
        StdOut.println("95% confidence interval = [" + PS.confidenceLo() + ", " + PS.confidenceHi() + "]");
    }

}
