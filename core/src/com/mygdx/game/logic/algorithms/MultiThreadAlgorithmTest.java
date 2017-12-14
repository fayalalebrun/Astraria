package com.mygdx.game.logic.algorithms;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 12/13/17   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.algorithms.threads.VelocityVerletTest;

import java.util.Vector;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class MultiThreadAlgorithmTest extends NBodyAlgorithm{
    private int amountOfThreads;
    private ExecutorService executorService;
    private Vector<Body> newBodies;

    public MultiThreadAlgorithmTest(Vector<Body> bodies){
        super(bodies);
        this.amountOfThreads = Runtime.getRuntime().availableProcessors();
        //this.amountOfThreads = 10;

        executorService = Executors.newFixedThreadPool(this.amountOfThreads);
        this.newBodies = new Vector<Body>();
    }

    @Override
    protected void runAlgorithm() {

        int toProcess = 1000;

        System.out.println("Starting...");
        System.out.println("Data to process: "+toProcess+"\nTime per data: 0.1 seconds");
        System.out.println("Expected time with single thread: "+(toProcess*0.1)+" seconds");
        System.out.println("Threads (automatic select): "+amountOfThreads);

        long a = System.nanoTime();

        CountDownLatch countDownLatch = new CountDownLatch(toProcess);

        for (int i = 0; i < toProcess; i++){
            executorService.submit(new VelocityVerletTest(countDownLatch, this));
        }


        try {
            countDownLatch.await();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        bodies.clear();
        bodies.addAll(newBodies);
        newBodies.clear();

        long b = System.nanoTime();

        System.out.println("==========================================");
        System.out.println("");
        System.out.println("Completed in: "+((b-a)/(double) 1000000000)+" seconds");
        executorService.shutdown();


    }

    @Override
    protected void endThreads() {
        executorService.shutdown();
    }

    public synchronized void addResult(Body body){
        newBodies.add(body);
    }

    public static void main(String args[]) {
        (new Thread(new MultiThreadAlgorithmTest(new Vector<Body>()))).start();
    }


}