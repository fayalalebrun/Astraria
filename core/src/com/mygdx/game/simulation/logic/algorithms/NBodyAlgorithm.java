package com.mygdx.game.simulation.logic.algorithms;

import com.mygdx.game.simulation.logic.Body;

import java.util.Vector;

/**
 * Created by fraayala19 on 12/12/17.
 */
public abstract class NBodyAlgorithm implements Runnable{

    private static final boolean PRINT_CALC_PER_SEC = false;

    protected final Vector<Body> bodies;

    protected final Vector<Body> toRemove;

    protected final Vector<Body> toAdd;

    public boolean terminate;

    private double lastTime, lastDelta;


    private int calcSec;
    private double timeSinceCalcSec;

    public NBodyAlgorithm() {
        this.bodies = new Vector<Body>();
        toRemove = new Vector<Body>();
        toAdd  = new Vector<Body>();
    }

    @Override
    public void run() {
        while (!terminate){

            runAlgorithm();

            if(PRINT_CALC_PER_SEC) {
                calcSec++;
                timeSinceCalcSec += lastDelta;
                if (timeSinceCalcSec >= 1) {
                    timeSinceCalcSec = 0;
                    System.out.println(calcSec);
                    calcSec = 0;
                }
            }

            bodies.removeAll(toRemove);
            toRemove.clear();
            bodies.addAll(toAdd);
            toAdd.clear();

        }
    }

    protected abstract void runAlgorithm();

    protected double getDelta(){
        double currTime = (double)System.nanoTime()/1000000000.0;
        if(lastTime==0){
            lastTime = currTime;
        }
        double temp = currTime - lastTime;
        lastTime = currTime;
        lastDelta = temp;
        return Math.min(temp,0.1);
    }

    protected double cb(double x){
        return x*x*x;
    }

    protected double sq(double x){
        return x*x;
    }

    public void addBody(Body body){
        toAdd.add(body);
    }

    public void removeBody(Body body){
        toRemove.add(body);
    }
}
