package com.mygdx.game.logic;

import com.badlogic.gdx.graphics.Camera;
import com.badlogic.gdx.graphics.PerspectiveCamera;
import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.graphics.g3d.model.Node;
import com.badlogic.gdx.math.Quaternion;
import com.badlogic.gdx.math.Vector3;
import com.mygdx.game.logic.helpers.Units;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class DetailedBody extends Body{

    ModelInstance instance;
    protected double radius;
    protected String name;
    private double auRadius;
    private Vector3 pos;

    public DetailedBody(double mass, double x, double y, double z, Model model, double radius, String name) {
        super(mass, x, y, z);
        this.instance = new ModelInstance(model);
        this.pos = new Vector3();
        this.radius = radius;
        this.name = name;
        auRadius = Units.mToAU(radius);
        instance.transform.scale((float)auRadius*2,(float)auRadius*2,(float)auRadius*2);
    }

    public void render(){
        instance.transform.getTranslation(pos);
        instance.transform.trn(-pos.x, -pos.y, -pos.z);
        instance.transform.trn((float)x, (float)y, (float)z);
    }

    public void centerCamera(Camera cam){
        cam.position.set((float)x+((float)auRadius*4),(float)y, (float)z);
        cam.lookAt((float)x,(float)y,(float)z);
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

    public Vector3 getPos() {
        return pos;
    }
}
