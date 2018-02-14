package com.company.algorithm;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/12/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import com.company.algorithm.helpers.Units;

import java.util.concurrent.CountDownLatch;


    public class VelocityVerlet implements Runnable {
        private CountDownLatch latch;
        private int thisBody;
        private float temporary, ax, ay, az;
        private MultiThreadAlgorithm multiThreadParent;
        private float delta;

        private float smoothingFactor;

        public VelocityVerlet(CountDownLatch latch, MultiThreadAlgorithm multiThreadParent, int thisBody, float delta, float smoothingFactor) {
            this.latch = latch;
            this.thisBody = thisBody;
            this.multiThreadParent = multiThreadParent;
            this.delta = delta;
            this.smoothingFactor = smoothingFactor;
        }

        @Override
        public void run() {

            ax = 0;
            ay = 0;
            az = 0;



            float pX = multiThreadParent.getX()[thisBody];
            float pY = multiThreadParent.getY()[thisBody];
            float pZ = multiThreadParent.getZ()[thisBody];



            pX = pX + (multiThreadParent.getVx()[thisBody] * delta) + (0.5f * multiThreadParent.getAx()[thisBody] *square(delta));
            pY = pY + (multiThreadParent.getVy()[thisBody] * delta) + (0.5f * multiThreadParent.getAy()[thisBody] *square(delta));
            pZ = pZ + (multiThreadParent.getVz()[thisBody] * delta) + (0.5f * multiThreadParent.getAz()[thisBody] *square(delta));


            multiThreadParent.getX2()[thisBody] = pX;
            multiThreadParent.getY2()[thisBody] = pY;
            multiThreadParent.getZ2()[thisBody] = pZ;



            ax = 0;
            ay = 0;
            az = 0;



            for (int k = 0; k < thisBody; k++) {
                parseAcceleration(k, pX, pY, pZ);
            }
            for (int k =thisBody + 1; k < multiThreadParent.getX().length; k++) {
                parseAcceleration(k, pX, pY, pZ);
            }

            ax *= Units.GRAV;
            ay *= Units.GRAV;
            az *= Units.GRAV;




            multiThreadParent.getVx2()[thisBody] = multiThreadParent.getVx()[thisBody] + (((multiThreadParent.getAx()[thisBody] + ax)/2)*delta);
            multiThreadParent.getVy2()[thisBody] = multiThreadParent.getVy()[thisBody] + (((multiThreadParent.getAy()[thisBody] + ay)/2)*delta);
            multiThreadParent.getVz2()[thisBody] = multiThreadParent.getVz()[thisBody] + (((multiThreadParent.getAz()[thisBody] + az)/2)*delta);


            multiThreadParent.getAx2()[thisBody] = ax;
            multiThreadParent.getAy2()[thisBody] = ay;
            multiThreadParent.getAz2()[thisBody] = az;



            latch.countDown();
        }

        private void parseAcceleration(int other, float pX, float pY, float pZ) {
            float oX = multiThreadParent.getX()[other];
            float oY = multiThreadParent.getY()[other];
            float oZ = multiThreadParent.getZ()[other];

            temporary = multiThreadParent.getM() / (cubed( (float) Math.sqrt(square(oX - pX) + square(oY - pY) + square(oZ - pZ))+smoothingFactor));


            ax += (oX - pX) * temporary;
            ay += (oY - pY) * temporary;
            az += (oZ - pZ) * temporary;

        }

        private float cubed(float number) {
            return number * number * number;
        }

        private float square(float number) {
            return number * number;
        }


    }


