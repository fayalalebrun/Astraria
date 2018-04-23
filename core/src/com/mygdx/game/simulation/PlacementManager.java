package com.mygdx.game.simulation;

import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.MousePicker;
import org.joml.Vector3d;
import org.joml.Vector3f;

/**
 * Created by fraayala19 on 4/23/18.
 */
public class PlacementManager {
    private boolean active = false;
    private SimulationObject simulationObject;
    private MousePicker mousePicker;

    Vector3d temp;

    public PlacementManager(MousePicker mousePicker) {
        this.mousePicker = mousePicker;

        temp = new Vector3d();
    }

    public void setActive(boolean active, SimulationObject simulationObject){
        this.active = active;
        this.simulationObject = simulationObject;
    }

    public void render(Camera camera, float delta){
        if(simulationObject!=null) {
            temp.set(mousePicker.getCurrentRay()).mul(simulationObject.getRadius() * 10);
            temp.add(camera.getPosition());
            simulationObject.setAbsPosition(temp.x, temp.y, temp.z);

            simulationObject.render(camera, 0f);
        }
    }
}
