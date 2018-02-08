package com.company.algorithm;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/2/18   ==
* =====================================================================
* ==      Project: Generator tool    ==
* =====================================================================

*/

import com.company.algorithm.helpers.AppendableObjectOutputStream;

import java.io.*;

public class AccelerationPatcher {


    private FileInputStream ifStream;
    private ObjectInputStream stream;
    private float average;

    public AccelerationPatcher(float average, File path){
        this.average = average;
        System.out.println(average);


        try {
            FileInputStream ifStream = new FileInputStream(path);
            ObjectInputStream stream = new ObjectInputStream(ifStream);

            stream.readShort();
            stream.readInt();
            stream.readFloat();

            this.ifStream = ifStream;
            this.stream = stream;


            appendMaxAndMin(getVariance(), path);



        }catch (Exception e){
            System.out.println("ERROR: "+e.getMessage());
        }




    }

    private float getVariance(){
        long k = 0;
        float variance = 0;
        try {
            while (ifStream.available()>16){

                stream.readFloat();
                stream.readFloat();
                stream.readFloat();

                float currAccel = stream.readFloat();

                variance = ((variance*k)+MultiThreadAlgorithm.square((currAccel - average)))/(k+1);
                k++;
            }

            variance = (float) Math.sqrt(variance);


            ifStream.close();
            stream.close();
            System.out.println(variance);
        }catch (Exception e){
            System.out.println("ERROR: "+e.getMessage());
        }

        return variance;

    }

    private void appendMaxAndMin(float variance, File path){

        System.out.println(path);

        try {
            FileOutputStream ofStream = new FileOutputStream(path, true);
            AppendableObjectOutputStream oStream = new AppendableObjectOutputStream(ofStream);

            oStream.writeFloat(average+variance);
            oStream.writeFloat(average-variance);


            System.out.println(variance);
            System.out.println(average+variance);
            System.out.println(average-variance);


            oStream.close();
            ofStream.close();

        }catch (Exception e){
            System.out.println("ERROR: ");
            e.printStackTrace();
        }

    }
}
