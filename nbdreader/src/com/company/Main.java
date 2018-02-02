package com.company;

import java.io.FileInputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.util.ArrayList;

public class Main {

    public static void main(String[] args) {
	// write your code here
try {

    FileInputStream fileStream = new FileInputStream("/Users/davrockenzahn19/orbit-simulator-2/initial conditions/sim_128.nbd");

    ObjectInputStream stream = new ObjectInputStream(fileStream);


    System.out.println(stream.readShort());
    System.out.println(stream.readInt());
    System.out.println(stream.readFloat());

    ArrayList<Float> k = new ArrayList<Float>();

    int l = 0;
    while (l < 600){

        float i = stream.readFloat();

    k.add(i);

    l++;


    }

    System.out.println(l);
    System.out.println(k.get(l-1));
    System.out.println(k.size());





}catch (Exception e){
            e.getMessage();
        }



    }
}
