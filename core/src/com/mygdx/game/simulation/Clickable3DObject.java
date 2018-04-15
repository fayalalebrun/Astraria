package com.mygdx.game.simulation;

import com.mygdx.game.simulation.renderer.Renderer;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.joml.Vector4f;

/**
 * Created by Fran on 4/15/2018.
 */
public class Clickable3DObject implements Comparable<Clickable3DObject>{
    private final Vector4f deviceCoords;
    private final Vector2f screenCoords;
    private final SimulationObject parent;

    public Clickable3DObject(SimulationObject parent) {
        this.parent = parent;
        deviceCoords = new Vector4f();
        screenCoords = new Vector2f();
    }

    public void updateCoords(Renderer renderer){
        Vector3f parentPos = parent.getPositionRelativeToCamera(renderer.getCamera());
        deviceCoords.set(renderer.worldSpaceToDeviceCoords(deviceCoords.set(parentPos.x,parentPos.y,parentPos.z,1.0f)));
        screenCoords.set(renderer.projectPoint(deviceCoords));
    }

    public float getZPos(){
        return deviceCoords.z;
    }

    public Vector2f getScreenCoords() {
        return screenCoords;
    }

    public SimulationObject getParent() {
        return parent;
    }

    public Vector4f getDeviceCoords() {
        return deviceCoords;
    }

    @Override
    public int compareTo(Clickable3DObject o) {
        return (int)(getZPos()*100000) - (int)(o.getZPos()*100000);
    }
}
