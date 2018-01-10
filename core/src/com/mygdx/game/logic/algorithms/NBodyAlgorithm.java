package com.mygdx.game.logic.algorithms;

import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.DetailedBody;

import java.util.Vector;

/**
 * Created by fraayala19 on 12/12/17.
 */
public abstract class NBodyAlgorithm implements Runnable{

    protected Vector<DetailedBody> bodies;
    private boolean terminate = false;
    private double lastTime;
    protected final Object lock;

    private double tmp;


    public NBodyAlgorithm(Vector<DetailedBody> bodies, Object lock) {
        this.bodies = bodies;

        this.lock = lock;
    }

    @Override
    public void run() {

        int calcSec = 0;
        double timeSinceCalcSec = 0;

        double average = 0;
        double times = 0;


        while (!terminate){
           runAlgorithm();
        // System.out.println("Algorithm done");

            //System.out.println(tmp);


            if(true) {
                calcSec++;
                timeSinceCalcSec += tmp;
                if (timeSinceCalcSec >= 1) {
                    timeSinceCalcSec = 0;
                    System.out.println("Calc: "+calcSec);

                    average = (average*times+calcSec)/(times+1);

                    System.out.println("Avg: "+average);



                    calcSec = 0;
                    times++;
                }
            }

        }

        endThreads();
    }


    protected abstract void runAlgorithm();

    private void terminate(){
        terminate = true;
    }

    protected abstract void endThreads();

    protected double getDelta(){
        double currTime = (double)System.nanoTime()/1000000000.0;
        if(lastTime==0){
            lastTime = currTime;
        }
        double temp = currTime - lastTime;
        lastTime = currTime;

        this.tmp = temp;

        return temp;
    }
}
