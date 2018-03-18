package com.mygdx.game.simulation;

import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Model;
import com.mygdx.game.simulation.renderer.Transformation;
import org.joml.Matrix4f;
import org.joml.Vector3d;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/17/2018.
 */
public class SimulationObject {
    private final Vector3d position, temp;
    private final Vector3f temp2;
    private final Model model;
    private final Vector3f rotation;

    public SimulationObject(double x, double y, double z, Model model) {
        position = new Vector3d(x,y,z);
        this.model = model;
        this.temp = new Vector3d();
        this.temp2 = new Vector3f();
        rotation = new Vector3f();
    }

    public void update(Camera cam){
        model.setPosition(temp2.set(temp.set(position).sub(cam.getPosition())));
    }

    public void render(Camera cam){
        model.render(cam);
    }
}
