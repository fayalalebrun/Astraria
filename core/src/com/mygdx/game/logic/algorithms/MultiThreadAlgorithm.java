package com.mygdx.game.logic.algorithms;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 12/13/17   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.algorithms.threads.VelocityVerlet;

import java.util.Vector;
import java.util.concurrent.*;

public class MultiThreadAlgorithm extends NBodyAlgorithm{
    private int amountOfThreads;
    private ExecutorService executorService;
    private double [] x;
    private double [] y;
    private double [] z;
    private double [] vx;
    private double [] vy;
    private double [] vz;
    private double [] ax;
    private double [] ay;
    private double [] az;
    private int lastSize;



    public MultiThreadAlgorithm(Vector<Body> bodies){
        super(bodies);
        this.amountOfThreads = Runtime.getRuntime().availableProcessors()-1;
        executorService = Executors.newFixedThreadPool(this.amountOfThreads);
        int k = bodies.size();
        x = new double[k];
        y = new double[k];
        z = new double[k];
        vx = new double[k];
        vy = new double[k];
        vz = new double[k];
        ax = new double[k];
        ay = new double[k];
        az = new double[k];
        lastSize  = k;

    }

    private void updateArraySize(){
        int k = bodies.size();
        x = new double[k];
        y = new double[k];
        z = new double[k];
        vx = new double[k];
        vy = new double[k];
        vz = new double[k];
        ax = new double[k];
        ay = new double[k];
        az = new double[k];
        lastSize = k;
    }

    @Override
    protected void runAlgorithm() {

        synchronized (bodies) {


            if (lastSize != bodies.size()) {
                updateArraySize();
            }


            CountDownLatch countDownLatch = new CountDownLatch(bodies.size());

            //

            double delta = this.getDelta();

            for (int i = 0; i < bodies.size(); i++) {

                executorService.submit(new VelocityVerlet(countDownLatch, this, i, delta));

            }


            try {
                countDownLatch.await();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }

            updateBodies();

        }

    }

    @Override
    protected void endThreads() {
        executorService.shutdown();
    }

    public double[] getX() {
        return x;
    }

    public double[] getY() {
        return y;
    }

    public double[] getZ() {
        return z;
    }

    public double[] getVx() {
        return vx;
    }

    public double[] getVy() {
        return vy;
    }

    public double[] getVz() {
        return vz;
    }

    public double[] getAx(){
        return ax;
    }

    public double[] getAy(){
        return ay;
    }

    public double[] getAz(){
        return az;
    }

    public Vector<Body> getBodies(){
        return bodies;
    }

    private void updateBodies(){

        int i = 0;
        for (Body current : bodies){
            current.setX(x[i]);
            current.setY(y[i]);
            current.setZ(z[i]);
            current.setvX(vx[i]);
            current.setvY(vy[i]);
            current.setvZ(vz[i]);
            current.setCurrAccelX(ax[i]);
            current.setCurrAccelY(ay[i]);
            current.setCurrAccelZ(az[i]);
            i++;
        }

    }
}