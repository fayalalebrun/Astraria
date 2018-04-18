package com.mygdx.game.simulation;

import com.badlogic.gdx.graphics.Color;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.*;

/**
 * Created by Fran on 4/18/2018.
 */
public class Planet extends SphericObject{
    public Planet(Renderer renderer, GLTexture texture, float radius, String name, Transformation transformation, Body body, Color orbitColor) {
        super(renderer, texture, renderer.getPlanetShader(), radius, name, transformation, body, orbitColor);
    }
}
