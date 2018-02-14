package com.mygdx.game.playback;

import com.badlogic.gdx.math.Vector3;
import com.sun.tools.javac.util.Pair;

import java.io.RandomAccessFile;
import java.util.Vector;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Created by fraayala19 on 2/13/18.
 */
public class PlayBackLoader implements Runnable {

    private String path;

    private RandomAccessFile randomAccessFile;
    private boolean terminate;

    private int numberOfBodies;
    private float bodyScale, minAccel, maxAccel, cycles;

    private ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>> frameMap;

    public PlayBackLoader(String path) {
        this.path = path;


        try {
            randomAccessFile = new RandomAccessFile(path, "r");

            short version = randomAccessFile.readShort();

            if(version==1){
                numberOfBodies = randomAccessFile.readInt();
                bodyScale = randomAccessFile.readFloat();
                long length = randomAccessFile.length();
                length-=length - 10 - 2;
                length/=4;
                cycles = length;
            }

        }catch (Exception e){
            e.printStackTrace();
        }
    }

    @Override
    public void run() {
        while(!terminate){

        }
    }
}
