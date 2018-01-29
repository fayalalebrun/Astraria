package com.company.files;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 1/13/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import java.io.File;
import java.io.FileOutputStream;
import java.io.ObjectOutputStream;
import java.io.OutputStream;
import java.util.concurrent.ConcurrentLinkedQueue;

public class BinWriter implements Runnable {

    private ConcurrentLinkedQueue<Float> queue;
    private boolean terminate;
    private ObjectOutputStream stream;

    private short version;
    private int bodies;
    private float scale;

    public BinWriter(short version, int bodies, float scale){
        queue = new ConcurrentLinkedQueue<Float>();
        terminate = false;

        this.version = version;
        this.bodies = bodies;
        this.scale = scale;
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
             stream.writeFloat(100f);
             stream.writeFloat(0f);


            }catch (Exception e){
            System.out.println(e.getMessage());
        }

        while (!terminate || !queue.isEmpty()){

            if (!queue.isEmpty()){
                try {
                    float f = queue.poll();
                    //System.out.println(f);
                    stream.writeFloat(f);
                }catch (Exception e){
                    System.out.println("ERROR: Unable to write: "+e.getMessage());
                }
            }
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
