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
}
