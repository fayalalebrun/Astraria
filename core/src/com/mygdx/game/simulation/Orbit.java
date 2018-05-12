package com.mygdx.game.simulation;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.utils.Queue;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Line;
import com.mygdx.game.simulation.renderer.Shader;
import org.joml.Vector3d;

import java.util.ArrayList;
import java.util.concurrent.ArrayBlockingQueue;

/**
 * Created by Fran on 4/14/2018.
 */
public class Orbit {
    private final static int MAX_POINTS = 500;

    private Line line;
    private float segmentLength;
    private final Vector3d lastPosition;
    private final Vector3d temp;
    private final ArrayBlockingQueue<Double> positions;
    private final float[] vertices;
    private final Shader shader;

    public Orbit(Color color, float segmentLength, Shader shader) {
        this.segmentLength = segmentLength;
        this.line = new Line(MAX_POINTS, color);
        lastPosition = new Vector3d();
        temp = new Vector3d();
        positions = new ArrayBlockingQueue<Double>(MAX_POINTS*3);

        vertices = new float[MAX_POINTS*3];
        this.shader = shader;
    }

    public void prepare(double x, double y, double z, Camera cam){
        while (positions.remainingCapacity()<10){
            positions.poll();
            positions.poll();
            positions.poll();
        }

        if(positions.size()<1){
            lastPosition.set(x,y,z);
            positions.add(x);
            positions.add(y);
            positions.add(z);
        }
        temp.set(x,y,z);
        temp.sub(lastPosition);
        if(Math.abs(temp.length())>segmentLength){
            lastPosition.set(x,y,z);
            positions.add(x);
            positions.add(y);
            positions.add(z);
        }


        fillArray(cam,x,y,z);
        line.updateBufferData(vertices,positions.size()/3);
    }

    public void render(){
        line.render(shader);
    }

    private void fillArray(Camera cam, double currX, double currY, double currZ){
        int i = 0, count = 0;
        Vector3d pos = new Vector3d();
        for(Double d : positions){
            if(count == 0){
                pos.x = d;
            } else if (count == 1){
                pos.y = d;
            } else if (count == 2){
                pos.z = d;
            }
            count ++;

            if(count == 3){
                count = 0;

                pos.sub(cam.getPosition());

                vertices[i] = (float)pos.x;
                i++;
                vertices[i] = (float) pos.y;
                i++;
                vertices[i] = (float)pos.z;
                i++;
            }
        }
        i-=3;
        pos.set(currX,currY,currZ);
        pos.sub(cam.getPosition());
        vertices[i] = (float)pos.x;
        i++;
        vertices[i] = (float) pos.y;
        i++;
        vertices[i] = (float)pos.z;
        i++;
    }


}
