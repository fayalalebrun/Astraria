package com.mygdx.game.simulation;

import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Model;
import com.mygdx.game.simulation.renderer.Shader;
import com.mygdx.game.simulation.renderer.Transformation;
import org.joml.Vector3d;
import org.joml.Vector3f;

/**
 * Created by Fran on 3/17/2018.
 */
public class SimulationObject implements Disposable{
    protected final Vector3d position, temp;
    private final Vector3f temp2;
    private final Model model;
    protected final Vector3f rotation;
    private float radius;
    private final String name;
    protected final Shader shader;
    protected final Transformation transformation;

    public SimulationObject(double x, double y, double z, Model model, Shader shader, float radius, String name, Transformation transformation) {
        position = new Vector3d(x,y,z);
        this.model = model;
        this.temp = new Vector3d();
        this.temp2 = new Vector3f();
        rotation = new Vector3f();
        this.radius = radius;
        this.name = name;
        this.shader = shader;
        this.transformation = transformation;
    }

    protected void update(Camera cam){
        model.setScale(radius);
        model.setPosition(getPositionRelativeToCamera(cam));
        model.setRotation(rotation.x,rotation.y,rotation.z);
    }

    public void render(Camera cam){
        update(cam);
        model.render(cam, shader);
    }


    public Vector3f getPositionRelativeToCamera(Camera cam){
        return temp2.set(temp.set(position).sub(cam.getPosition()));
    }

    public String getName() {
        return name;
    }

    public float getRadius() {
        return radius;
    }

    @Override
    public void dispose() {
        model.dispose();
    }

}
