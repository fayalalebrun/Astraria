package com.company.algorithm;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/11/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import com.company.screen.Menu;

import java.util.Random;
import java.util.Vector;

public abstract class ThreadOrganizer {

        private double lastCalc;
        private double lastAvg;

        private boolean terminate = false;
        private double lastTime;
        protected final Object lock;

        private double tmp;

    protected int duration;
    protected double l;
    protected double l2;

        public static boolean PRINT_CALC_SEC = true;


        public ThreadOrganizer(Object lock) {
            this.lock = lock;

        }

        public void run() {

            int calcSec = 0;
            double timeSinceCalcSec = 0;

            double average = 0;
            double times = 0;


            while (!terminate){
                runAlgorithm();
                // System.out.println("Algorithm done");

                //System.out.println(tmp);


                if(PRINT_CALC_SEC) {
                    calcSec++;
                    timeSinceCalcSec += tmp;
                    if (timeSinceCalcSec >= 1) {
                        timeSinceCalcSec = 0;
                        lastCalc = calcSec;

                        average = (average*times+calcSec)/(times+1);

                        lastAvg = average;



                        calcSec = 0;
                        times++;
                    }
                }

            }

            endThreads();
        }


        protected abstract void runAlgorithm();

        protected void terminate(){
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

        public void printStatus(){

            Menu.printHeader();

            System.out.println("Last cycles per second: "+lastCalc);
            System.out.println("Average cycles: "+lastAvg);
            System.out.println("");
            System.out.println("Generating, please wait... "+"[ "+ (float) (l / ((double) duration * 60D))*100+" % COMPLETE]");
            System.out.println("[PRESS CTR+Z TO CANCEL]");
        }
    }


