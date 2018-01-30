package com.company.files;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/13/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import com.company.algorithm.MultiThreadAlgorithm;
import com.company.algorithm.VelocityVerlet;

import java.io.File;
import java.io.FileOutputStream;
import java.io.ObjectOutputStream;
import java.io.OutputStream;
import java.util.concurrent.ConcurrentLinkedQueue;

public class BinWriter implements Runnable {

    private ConcurrentLinkedQueue<Float> queue;
    private ConcurrentLinkedQueue<Float> accelqueue;
    private boolean terminate;
    private ObjectOutputStream stream;

    private short version;
    private int bodies;
    private float scale;

    private float maxAcceleration;
    private float minAcceleration;

    public BinWriter(short version, int bodies, float scale){
        queue = new ConcurrentLinkedQueue<Float>();
        accelqueue = new ConcurrentLinkedQueue<Float>();
        terminate = false;

        this.version = version;
        this.bodies = bodies;
        this.scale = scale;

        maxAcceleration = Float.MIN_VALUE;
        minAcceleration = Float.MAX_VALUE;
    }

    @Override
    public void run() {
        try {

        //    System.out.println(version);
          //  System.out.println(bodies);
            //System.out.println(scale);
            //System.out.println(100f);
            //System.out.println(0f);

             stream.writeShort(version);
             stream.writeInt(bodies);
             stream.writeFloat(scale);



            }catch (Exception e){
            System.out.println(e.getMessage());
        }

        int i = 0;

        while (!terminate || !queue.isEmpty()){

            if (!queue.isEmpty()){
                try {

                    if (i>=3){
                        i=0;

                        float ax = queue.poll();
                        float ay = queue.poll();
                        float az = queue.poll();

                        float currAcc = (float) Math.sqrt(MultiThreadAlgorithm.square(ax)+MultiThreadAlgorithm.square(ay)+MultiThreadAlgorithm.square(az));

                        if (currAcc > maxAcceleration){
                            maxAcceleration = currAcc;
                        }
                        if (currAcc < minAcceleration){
                            minAcceleration = currAcc;
                        }

                        stream.writeFloat(currAcc);
                    }else {
                        float f = queue.poll();
                        //System.out.println(f);
                        stream.writeFloat(f);
                        i++;
                    }



                }catch (Exception e){
                    System.out.println("ERROR: Unable to write: "+e.getMessage());
                }
            }
        }

        try {
            stream.writeFloat(maxAcceleration);
            stream.writeFloat(minAcceleration);
        }catch (Exception e){
            System.out.println(e.getMessage());
        }


    }

    public void terminate(){
        terminate=true;
        try {
            stream.close();

        }catch (Exception e){
            System.out.println(e.getMessage());
        }
    }

    public boolean setFile(File output){
        try {
            stream = new ObjectOutputStream(new FileOutputStream(output));

            return true;
        }catch (Exception e){
            System.out.println("ERROR: Output stream could not locate file: "+e.getMessage());
            return false;
        }
    }

    public ConcurrentLinkedQueue<Float> getQueue(){
        return queue;
    }




}
