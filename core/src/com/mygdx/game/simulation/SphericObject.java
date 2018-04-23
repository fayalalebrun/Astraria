package com.mygdx.game.simulation;

import com.badlogic.gdx.graphics.Color;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.*;

/**
 * Created by Fran on 4/18/2018.
 */
public class SphericObject extends SimulationObject{
    public SphericObject(Renderer renderer, GLTexture texture, Shader objectShader, float radius, String name, Transformation transformation, Body body, Color orbitColor) {
        super(new Sphere(renderer.getTransformation(), texture), objectShader, renderer.getLineShader(), radius, name, transformation, body, orbitColor);
    }

    @Override
    public String toSaveFile() {
        String s = "";
        s+="name: "+getName()+"\n";
        s+="radius: "+getRadius()+"\n";
        s+="mass: "+getMass()+"\n";
        s+="velocity: "+getXVel() + " " + getYVel() + " " + getZVel()+"\n";
        s+="position: "+getXPos()+" "+getYPos()+" "+getZPos()+"\n";
        s+="texture: "+ ((Sphere)model).getDiffuseTexture().fileHandle.path()+"\n";
        s+="orbit_color: "+orbitColor.r + " " + orbitColor.g + " " + orbitColor.b + " " + orbitColor.a + "\n";
        s+="rotation: "+Math.toDegrees(getInclination())+" "+Math.toDegrees(getAxisRightAscension())+" " +
                Math.toDegrees(getRotationPeriod()) + " " + 0 + "\n";
        return s;
    }
}
