package com.mygdx.game.simulation.renderer;

import org.joml.Vector3d;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/18/2018.
 */
public class PointLight implements LightEmitter{
    final Vector3f position, temp, temp2;

    public PointLight(float x, float y, float z) {
        position = new Vector3f(x, y, z);
        temp = new Vector3f();
        temp2 = new Vector3f();
    }


    @Override
    public Vector3f getLightPosition(Camera camera, Transformation transformation) {
        return transformation.getViewMatrix(camera).transformProject(temp.set(position).sub(temp2.set(camera.getPosition())));
    }

    @Override
    public Vector3d getAbsolutePosition() {
        return new Vector3d(position);
    }

    @Override
    public Vector3f getAmbient() {
        return temp.set(0.05f,0.05f,0.05f);
    }

    @Override
    public Vector3f getDiffuse() {
        return temp.set(0.8f,0.8f,0.8f);
    }

    @Override
    public Vector3f getSpecular() {
        return temp.set(1.0f,1.0f,1.0f);
    }
}
