package com.company;

import java.io.FileInputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;

public class Main {

    public static void main(String[] args) {
	// write your code here
try {
    ObjectInputStream stream = new ObjectInputStream(new FileInputStream("/Users/davrockenzahn19/orbit-simulator-2/Generator tool/100%functional.nbd"));


    System.out.println(stream.readShort());
    System.out.println(stream.readInt());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());

    System.out.println();
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());

    System.out.println();
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());

    System.out.println();
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());

    System.out.println();
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());
    System.out.println(stream.readFloat());




}catch (Exception e){
            e.getMessage();
        }



    }
}
