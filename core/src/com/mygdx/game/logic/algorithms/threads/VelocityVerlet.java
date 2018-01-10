package com.mygdx.game.logic.algorithms.threads;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 12/13/17   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.mygdx.game.logic.Body;
import com.mygdx.game.logic.DetailedBody;
import com.mygdx.game.logic.algorithms.MultiThreadAlgorithm;
import com.mygdx.game.logic.helpers.Units;

import java.util.Vector;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.CountDownLatch;

public class VelocityVerlet implements Runnable {
    private CountDownLatch latch;
    private int thisBody;
    private double temporary, ax, ay, az;
    private MultiThreadAlgorithm multiThreadParent;
    private double delta;

    public VelocityVerlet(CountDownLatch latch, MultiThreadAlgorithm multiThreadParent, int thisBody, double delta) {
        this.latch = latch;
        this.thisBody = thisBody;
        this.multiThreadParent = multiThreadParent;
        this.delta = delta;
    }

    @Override
    public void run() {



        DetailedBody current = multiThreadParent.getBodies().elementAt(thisBody);

        //Verlet Algorithm



        //double delta = 0; //provisional delta

        //Get time
            ax = 0;
            ay = 0;
            az = 0;



            double pX = Units.AUToM(current.getX());
            double pY = Units.AUToM(current.getY());
            double pZ = Units.AUToM(current.getZ());



            Vector<DetailedBody> bodies = multiThreadParent.getBodies();


            //First time procedure?
            if (!current.getAccelInit()) {


                //No indexing for safe list
                //Find other way of initializing accelerations

                for (int l = 0; l < bodies.indexOf(current); l++) {
                    parseAcceleration(bodies.get(l), pX, pY, pZ);
                }
                for (int l = bodies.indexOf(current) + 1; l < bodies.size(); l++) {
                    parseAcceleration(bodies.get(l), pX, pY, pZ);
                }


                    ax *= Units.GRAV;
                    ay *= Units.GRAV;
                    az *= Units.GRAV;


                   current.setCurrAccelX(ax);
                   current.setCurrAccelY(ay);
                   current.setCurrAccelZ(az);
                   // multiThreadParent.getAy()[thisBody] = ay;
                  //  multiThreadParent.getAz()[thisBody] = az;

                    current.setAccelInitTrue();


            }



            pX = pX + (current.getvX() * delta) + (0.5*current.getCurrAccelX() *square(delta));
            pY = pY + (current.getvY() * delta) + (0.5*current.getCurrAccelY() *square(delta));
            pZ = pZ + (current.getvZ() * delta) + (0.5*current.getCurrAccelZ() *square(delta));


            multiThreadParent.getX()[thisBody] = Units.mToAU(pX);
            multiThreadParent.getY()[thisBody] = Units.mToAU(pY);
            multiThreadParent.getZ()[thisBody] = Units.mToAU(pZ);


            //new positions saved into arrays, bodies still hold old positions

            ax = 0;
            ay = 0;
            az = 0;



            for (int k = 0; k < bodies.indexOf(current); k++) {
                parseAcceleration(bodies.get(k), pX, pY, pZ);
            }
            for (int k = bodies.indexOf(current) + 1; k < bodies.size(); k++) {
                parseAcceleration(bodies.get(k), pX, pY, pZ);
            }
            ax *= Units.GRAV;
            ay *= Units.GRAV;
            az *= Units.GRAV;




            multiThreadParent.getVx()[thisBody] = current.getvX() + (((current.getCurrAccelX() + ax)/2)*delta);
            multiThreadParent.getVy()[thisBody] = current.getvY() + (((current.getCurrAccelY() + ay)/2)*delta);
            multiThreadParent.getVz()[thisBody] = current.getvZ() + (((current.getCurrAccelZ() + az)/2)*delta);



            multiThreadParent.getAx()[thisBody] = ax;
            multiThreadParent.getAy()[thisBody] = ay;
            multiThreadParent.getAz()[thisBody] = az;




        latch.countDown();
    }

    private void parseAcceleration(Body other, double pX, double pY, double pZ) {
        double oX = Units.AUToM(other.getX());
        double oY = Units.AUToM(other.getY());
        double oZ = Units.AUToM(other.getZ());

        temporary = other.getMass() / cubed(Math.sqrt(square(oX - pX) + square(oY - pY) + square(oZ - pZ)));


        ax += (oX - pX) * temporary;
        ay += (oY - pY) * temporary;
        az += (oZ - pZ) * temporary;

    }

    private double cubed(double number) {
        return number * number * number;
    }

    private double square(double number) {
        return number * number;
    }


}
