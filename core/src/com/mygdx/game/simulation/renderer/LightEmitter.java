package com.mygdx.game.simulation.renderer;

import org.joml.Vector3d;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/18/2018.
 */
public interface LightEmitter {

    public Vector3f getLightPosition(Camera camera);

    public Vector3f getAmbient();

    public Vector3f getDiffuse();

    public Vector3f getSpecular();
}
