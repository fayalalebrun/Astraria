package com.mygdx.game.logic;

import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.ModelInstance;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class DetailedBody extends Body{

    ModelInstance instance;
    protected double radius;
    protected String name;

    public DetailedBody(double mass, double x, double y, double z, Model model, double radius, String name) {
        super(mass, x, y, z);
        this.instance = new ModelInstance(model);
        this.radius = radius;
        this.name = name;
    }

    public ModelInstance getInstance() {
        return instance;
    }

    public double getRadius() {
        return radius;
    }

    public String getName() {
        return name;
    }
}
