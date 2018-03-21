package com.mygdx.game.simulation;

import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Model;
import com.mygdx.game.simulation.renderer.Transformation;
import org.joml.Matrix4f;
import org.joml.Vector3d;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/17/2018.
 */
public class SimulationObject implements Disposable{
    private final Vector3d position, temp;
    private final Vector3f temp2;
    private final Model model;
    private final Vector3f rotation;
    private float size;
    private final String name;

    public SimulationObject(double x, double y, double z, Model model, float size, String name) {
        position = new Vector3d(x,y,z);
        this.model = model;
        this.temp = new Vector3d();
        this.temp2 = new Vector3f();
        rotation = new Vector3f();
        this.size = size;
        this.name = name;
    }

    private void update(Camera cam){
        model.setScale(size);
        model.setPosition(getPositionRelativeToCamera(cam));
    }

    public void render(Camera cam){
        update(cam);
        model.render(cam);
    }


    public Vector3f getPositionRelativeToCamera(Camera cam){
        return temp2.set(temp.set(position).sub(cam.getPosition()));
    }

    public String getName() {
        return name;
    }

    @Override
    public void dispose() {
        model.dispose();
    }
}
