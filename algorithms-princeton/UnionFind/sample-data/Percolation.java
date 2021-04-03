package xyz.livexia;
import java.lang.*;
import edu.princeton.cs.algs4.StdIn;
import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.StdRandom;
import edu.princeton.cs.algs4.StdStats;
import edu.princeton.cs.algs4.WeightedQuickUnionUF;

public class Percolation {
    private int[][] g;
    private int count;
    private WeightedQuickUnionUF UF;

    public Percolation(int n){
        if (n < 0){
            throw new IllegalArgumentException("outside its prescribed range");
        }
        count = n;
        g = new int[n+1][n+1];
        UF = new WeightedQuickUnionUF(n*n+2);
        for(int i=1; i< n+1; i++){
            for(int j=1; j< n+1; j++){
                g[i][j] = 0;
            }
        }
    }

    public void open(int row, int col){
        g[row][col] = 1;
        if(row == 1)
            UF.union(0, (row-1)*count+col);
        if (row == count)
            UF.union(count*count+1, (row-1)*count+col);
        try {
            if (isOpen(row - 1, col))
                UF.union((row - 1) * count + col, (row - 2) * count + col);
            if (isOpen(row + 1, col))
                UF.union((row - 1) * count + col, row * count + col);
            if (isOpen(row, col - 1))
                UF.union((row - 1) * count + col, (row - 2) * count + col - 1);
            if (isOpen(row, col + 1))
                UF.union((row - 1) * count + col, (row - 2) * count + col + 1);
        }catch (IllegalArgumentException e){
            StdOut.println(e);
        }
    }

    public boolean isOpen(int row, int col){
        if(row>= 1 && row<=count && col>=1 && col<=count)
            return g[row][col] == 1;
        else {
            throw new IllegalArgumentException("outside its prescribed range");
        }
    }

    public boolean isFull(int row, int col){
        return UF.connected(0,(row-1)*count+col);
    }

    public int numberOfOpenSites(){
        int sum = 0;
        for(int i=0; i< count; i++){
            for(int j=0; j< count; j++){
                if(isOpen(i,j))
                    sum++;
            }
        }
        return sum;
    }

    public boolean percolates(){
        return UF.connected(0, count*count+1);
    }

    public static void main(String[] args) {
        // write your code here
        int N = StdIn.readInt();
        Percolation percolation = new Percolation(N);
        while (!StdIn.isEmpty()){
            int row = StdIn.readInt();
            int col = StdIn.readInt();
            percolation.open(row, col);
            StdOut.println("isPercolates: " + percolation.percolates());
        }


    }
}
