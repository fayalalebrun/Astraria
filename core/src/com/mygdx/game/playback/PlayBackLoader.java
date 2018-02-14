package com.mygdx.game.playback;

import com.badlogic.gdx.math.Vector3;
import javafx.util.Pair;

import java.io.IOException;
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

    private int numberOfBodies, cycles;
    private float bodyScale, minAccel, maxAccel;
    private volatile int firstFrame, currentFrame, lastFrame = -1;

    private ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>> frameMap;

    public PlayBackLoader(String path) {
        this.path = path;
        frameMap = new  ConcurrentHashMap<Integer, Vector<Pair<Vector3, Float>>>();
        try {
            randomAccessFile = new RandomAccessFile(path, "r");
            randomAccessFile.skipBytes(9);
            short version = randomAccessFile.readShort();

            if(version==1){
                numberOfBodies = randomAccessFile.readInt();
                bodyScale = randomAccessFile.readFloat();
                long length = randomAccessFile.length();
                length = length - 27;
                length/=16;
                cycles = (int)length;
                System.out.println(randomAccessFile.length());
                System.out.println((length*16)+27);

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
            synchronized (this) {
                if (currentFrame > firstFrame) {
                    frameMap.remove(firstFrame);
                    firstFrame++;
                }

                if (Runtime.getRuntime().freeMemory() > 30000000 && lastFrame < cycles) {
                    lastFrame++;

                    Vector<Pair<Vector3, Float>> frame = new Vector<Pair<Vector3, Float>>();

                    long pointer = 19;
                    pointer += lastFrame * numberOfBodies * 16;
                    try {
                        randomAccessFile.seek(pointer);

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
                }
            }
        }

        try {
            randomAccessFile.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public Vector<Pair<Vector3,Float>> requestFrame(int frame){
            currentFrame = frame;
            if (!(frame >= firstFrame && frame <= lastFrame)) {
                for (int i = firstFrame; i <= lastFrame; i++) {
                    frameMap.remove(i);
                }
                firstFrame = frame;
                lastFrame = frame-1;
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
