import edu.princeton.cs.algs4.In;
import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.WeightedQuickUnionUF;

// way2.Use both virtual bottom and top, backwash exists
// score 95/100
//Correctness:  30/33 tests passed
//Memory:       8/8 tests passed
//Timing:       20/20 tests passed
public class Percolation {
    private boolean[][] g;
    private int count;
    private int opensites;
    private WeightedQuickUnionUF weightedQuickUnionUF;

    public Percolation(int n) {
        if (n <= 0) {
            throw new IllegalArgumentException("outside its prescribed range");
        }
        count = n;
        g = new boolean[n + 1][n + 1];
        weightedQuickUnionUF = new WeightedQuickUnionUF(n * n + 2);
        for (int i = 1; i < n + 1; i++) {
            for (int j = 1; j < n + 1; j++) {
                g[i][j] = false;
            }
        }
    }

    public void open(int row, int col) {
        if (!isOpen(row, col)) {
            g[row][col] = true;
            if (row == 1)
                weightedQuickUnionUF.union(0, (row - 1) * count + col);
            if (row == count)
                weightedQuickUnionUF.union(count*count+1, (row - 1) * count + col);
            if (row>1 && isOpen(row - 1, col))
                weightedQuickUnionUF.union((row - 1) * count + col, (row - 2) * count + col);
            if (row<count && isOpen(row + 1, col))
                weightedQuickUnionUF.union((row - 1) * count + col, row * count + col);
            if (col>1 && isOpen(row, col - 1))
                weightedQuickUnionUF.union((row - 1) * count + col, (row - 1) * count + col - 1);
            if (col < count && isOpen(row, col + 1))
                weightedQuickUnionUF.union((row - 1) * count + col, (row - 1) * count + col + 1);
            opensites += 1;
        }
    }

    public boolean isOpen(int row, int col) {
        if (row >= 1 && row <= count && col >= 1 && col <= count)
            return g[row][col];
        else {
            throw new IllegalArgumentException(row + "," + col + " outside its prescribed range");
        }
    }

    public boolean isFull(int row, int col) {
        if (row >= 1 && row <= count && col >= 1 && col <= count)
            return weightedQuickUnionUF.connected(0, (row - 1) * count + col);
        else {
            throw new IllegalArgumentException(row + "," + col + " outside its prescribed range");
        }
    }

    public int numberOfOpenSites() {
        return opensites;
    }

    public boolean percolates() {
        return weightedQuickUnionUF.connected(0, count*count+1);
    }

    public static void main(String[] args) {
        In in = new In(args[0]);      // input file
        int n = in.readInt();
        Percolation percolation = new Percolation(n);
        while (!in.isEmpty()) {
            int row = in.readInt();
            int col = in.readInt();
            percolation.open(row, col);
            StdOut.println("isPercolates: " + percolation.percolates());
        }
    }
}
