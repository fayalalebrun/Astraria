package com.mygdx.game.logic.algorithms;

import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.DetailedBody;

import java.util.Vector;

/**
 * Created by fraayala19 on 12/12/17.
 */
public abstract class NBodyAlgorithm implements Runnable{

    protected  Vector<DetailedBody> bodies;
    private boolean terminate = false;
    private double lastTime;


    public NBodyAlgorithm(Vector<DetailedBody> bodies) {
        this.bodies = bodies;
    }

    @Override
    public void run() {
        if(!terminate){
            runAlgorithm();
        } else {
            endThreads();
        }
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
        return temp;
    }
}
