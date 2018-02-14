package com.mygdx.game.playback;

import com.badlogic.gdx.math.Vector3;

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
    private float bodyScale, minAccel, maxAccel;

    private ConcurrentHashMap<Integer, Vector<Vector3>> frameMap;

    public PlayBackLoader(String path) {
        this.path = path;
        

        try {
            randomAccessFile = new RandomAccessFile(path, "r");
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
