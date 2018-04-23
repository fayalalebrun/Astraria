package com.mygdx.game.simulation;

import com.mygdx.game.simulation.logic.helpers.Units;
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

    private double objectSpeed = 0;

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
        if(simulationObject!=null&&active) {
            temp.set(mousePicker.getCurrentRay()).mul(simulationObject.getRadius() * 10);
            temp.add(camera.getPosition());
            simulationObject.setAbsPosition(temp.x, temp.y, temp.z);

            simulationObject.render(camera, 0f);
        }
    }

    public void place(Camera camera, SimulationScreen simulationScreen){
        temp.set(mousePicker.getCurrentRay()).mul(simulationObject.getRadius() * 10);
        temp.add(camera.getPosition());
        simulationObject.setXPos(Units.kmToM(temp.x));
        simulationObject.setYPos(Units.kmToM(temp.z));
        simulationObject.setZPos(Units.kmToM(temp.y));

        temp.set(mousePicker.getCurrentRay()).mul(objectSpeed);
        simulationObject.setXVel(Units.kmToM(temp.x));
        simulationObject.setYVel(Units.kmToM(temp.z));
        simulationObject.setZVel(Units.kmToM(temp.y));

        simulationScreen.addObject(simulationObject);
        active = false;
        simulationObject = null;
    }

    public void setObjectSpeed(double objectSpeed) {
        this.objectSpeed = objectSpeed;
    }

    public boolean isActive() {
        return active;
    }
}
