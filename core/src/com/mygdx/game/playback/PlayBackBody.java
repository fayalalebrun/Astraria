package com.mygdx.game.playback;

import com.badlogic.gdx.graphics.Camera;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.graphics.g3d.attributes.ColorAttribute;
import com.badlogic.gdx.graphics.g3d.decals.Decal;
import com.badlogic.gdx.math.Vector3;

import java.util.ArrayList;
import java.util.Random;

/**
 * Created by fraayala19 on 1/11/18.
 */
public class PlayBackBody {
    private ArrayList<Vector3> positions;
    private ArrayList<Float> acceleration;
    private Color color;
    private Decal decal;
    private Vector3 temp1, temp2;
    private Random rand;

    public PlayBackBody(Decal decal, float size) {
        positions = new ArrayList<Vector3>();
        acceleration = new ArrayList<Float>();
        temp1 = new Vector3();
        temp2 = new Vector3();
        this.decal = decal;
        decal.setScale(0.01f);
        //rand = new Random();
        //color = new Color(rand.nextFloat(),rand.nextFloat(),rand.nextFloat(),1);
    }

    public void addPosition(Vector3 pos){
        positions.add(pos);
    }

    public void changeColor(Color color){
        this.color = color;
    }


    public boolean setFrame(int frame, Camera cam, float camDistance){
        if(frame>=positions.size()){
            return false;
        }
        setPosition(frame, cam, camDistance);
        return true;
    }

    public void addAcceleration(float accel){
        acceleration.add(accel);
    }

    private void setPosition(int frame, Camera cam, float camDistance){
        //decal.setColor(new Color(rand.nextFloat(),rand.nextFloat(),rand.nextFloat(),1));
        temp1 = positions.get(frame).cpy().sub(cam.position);
        temp1.nor();
        temp2 = new Vector3(temp1.x*camDistance,temp1.y*camDistance,temp1.z*camDistance);
        decal.setPosition(new Vector3(cam.position.x+temp2.x,cam.position.y+temp2.y,cam.position.z+temp2.z));
        decal.lookAt(cam.position,cam.up);
    }


    public ArrayList<Vector3> getPositions() {
        return positions;
    }

    public Decal getDecal() {
        return decal;
    }
}
