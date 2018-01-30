package com.mygdx.game.playback;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Camera;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.g2d.Sprite;
import com.badlogic.gdx.graphics.g2d.SpriteBatch;
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
    private Sprite sprite;
    private Vector3 temp1, temp2;
    private Random rand;

    public PlayBackBody(Sprite sprite, float size) {
        positions = new ArrayList<Vector3>();
        acceleration = new ArrayList<Float>();
        temp1 = new Vector3();
        temp2 = new Vector3();
        this.sprite = sprite;
        sprite.setScale(0.05f);
        rand = new Random();
        color = new Color(rand.nextFloat(),rand.nextFloat(),rand.nextFloat(),1);

    }

    public void addPosition(Vector3 pos){
        positions.add(pos);
    }

    public void changeColor(Color color){
        this.color = color;
    }


    public void setFrame(int frame, Camera cam, SpriteBatch batch){
        if(frame<positions.size()){
            setPosition(frame, cam);
            if(cam.frustum.pointInFrustum(positions.get(frame))){
                draw(batch);
            }
        }else{
            if(cam.frustum.pointInFrustum(positions.get(positions.size()-1))){
                setPosition(positions.size()-1,cam);
                draw(batch);
            }
        }

    }

    public void draw(SpriteBatch batch){
        sprite.draw(batch);
    }

    public void addAcceleration(float accel){
        acceleration.add(accel);
    }

    private void setPosition(int frame, Camera cam){
        sprite.setColor(color);
        Vector3 pos = positions.get(frame).cpy();
        cam.project(pos);
        //pos.x = pos.x - (sprite.getWidth()*sprite.getScaleX())*0.5f;
        //pos.y = pos.y - (sprite.getHeight()*sprite.getScaleY())*0.5f;
        //pos.x -= Gdx.graphics.getWidth()/2;
        //pos.y -= Gdx.graphics.getHeight()/2;
        System.out.println(pos.x+" "+pos.y);
        System.out.println(Gdx.graphics.getWidth()+" "+Gdx.graphics.getHeight());
        sprite.setPosition(pos.x,pos.y);
    }


    public ArrayList<Vector3> getPositions() {
        return positions;
    }

    public Sprite getSprite() {
        return sprite;
    }
}
