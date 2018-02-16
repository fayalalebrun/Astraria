package com.mygdx.game.playback;

import com.badlogic.gdx.math.Vector3;
import net.dermetfan.utils.Pair;

import java.io.IOException;
import java.io.RandomAccessFile;
import java.util.Vector;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Created by fraayala19 on 2/13/18.
 */
public class PlayBackLoader implements Runnable {

    private String path;

    private RandomAccessFile randomAccessFile;
    private boolean terminate;

    private int numberOfBodies, cycles;
    private float bodyScale, minAccel, maxAccel;
    private int firstFrame, currentFrame, lastFrame = -1;

    private ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>> frameMap;

    public PlayBackLoader(String path) {
        this.path = path;
        frameMap = new  ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>>();
        try {
            randomAccessFile = new RandomAccessFile(path, "r");
            short version = randomAccessFile.readShort();


            if(version==1){
                numberOfBodies = randomAccessFile.readInt();
                bodyScale = randomAccessFile.readFloat();
                long length = randomAccessFile.length();
                length = length - 18;
                length/=16*numberOfBodies;
                cycles = (int)length;

                randomAccessFile.seek(randomAccessFile.length()-8);

                maxAccel = randomAccessFile.readFloat();
                minAccel = randomAccessFile.readFloat();

            }

        }catch (Exception e){
            e.printStackTrace();
        }
    }

    @Override
    public void run() {
        while(!terminate){

                if (Runtime.getRuntime().freeMemory() > 30000000 && lastFrame < cycles) {

                    lastFrame+=1;


                    Vector<Pair<Vector3, Float>> frame = new Vector<Pair<Vector3, Float>>();

                    long pointer = 10;

                    pointer += (long)lastFrame * (long)numberOfBodies * 16;


                    try {
                        if(pointer!=randomAccessFile.getFilePointer()) {
                            randomAccessFile.seek(pointer);
                        }

                        for (int i = 0; i < numberOfBodies; i++) {
                            float x = randomAccessFile.readFloat() * 100;
                            float y = randomAccessFile.readFloat() * 100;
                            float z = randomAccessFile.readFloat() * 100;
                            float accel = randomAccessFile.readFloat();

                            frame.add(new Pair<Vector3, Float>(new Vector3(x, y, z), accel));
                        }
                        frameMap.put(lastFrame, frame);
                    } catch (Exception e) {
                        e.printStackTrace();
                    }

            } else if (lastFrame < cycles&&firstFrame<currentFrame){

                    ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>> newMap = new ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>>();

                        for(int i = currentFrame; i<=lastFrame;i++){
                            newMap.put(i,frameMap.get(i));
                        }

                        frameMap = newMap;

                    System.gc();

                        firstFrame = currentFrame;
                }
        }

        try {
            randomAccessFile.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public Vector<Pair<Vector3,Float>> requestFrame(int frame){

        currentFrame=frame;
        if (frame < firstFrame || frame > lastFrame) {

            frameMap = new ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>>();
            firstFrame=frame;
            lastFrame=frame;
            System.gc();
        }

        if (frameMap.containsKey(frame)) {
            return frameMap.get(frame);
        } else {
            return null;
        }

    }

    public void terminate(){
        terminate = true;
    }

    public int getNumberOfBodies() {
        return numberOfBodies;
    }

    public float getBodyScale() {
        return bodyScale;
    }

    public float getMinAccel() {
        return minAccel;
    }

    public float getMaxAccel() {
        return maxAccel;
    }

    public int getCycles() {
        return cycles;
    }
}
