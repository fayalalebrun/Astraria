package com.company;

import java.io.FileInputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.util.ArrayList;

public class Main {

    public static void main(String[] args) {
	// write your code here
try {

    FileInputStream fileStream = new FileInputStream("/Users/davrockenzahn19/orbit-simulator-2/initial conditions/mySimulation.nbd");

    ObjectInputStream stream = new ObjectInputStream(fileStream);


    System.out.println(stream.readShort());
    System.out.println(stream.readInt());
    System.out.println(stream.readFloat());

    ArrayList<Float> k = new ArrayList<Float>();

    int l = 0;
    while (stream.available()>0){

        stream.readFloat();


    l++;


    }

    System.out.println("j");







}catch (Exception e){
            e.printStackTrace();
        }



    }
}
