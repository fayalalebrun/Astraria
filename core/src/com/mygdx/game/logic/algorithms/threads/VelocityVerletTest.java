package com.mygdx.game.logic.algorithms.threads;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 12/13/17   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.algorithms.MultiThreadAlgorithmTest;

import java.util.concurrent.CountDownLatch;

public class VelocityVerletTest implements Runnable{
    private CountDownLatch latch;
    private MultiThreadAlgorithmTest resultParent;
    private Body body;

    public VelocityVerletTest(CountDownLatch latch, MultiThreadAlgorithmTest resutParent){
        this.latch = latch;
        this.resultParent = resutParent;
        this.body = new Body(0,0,0,0,0,0,0);
    }

    @Override
    public void run() {

        //Verlet Algorithm
        try {
            Thread.sleep(100);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        resultParent.addResult(body);
        latch.countDown();

    }
}