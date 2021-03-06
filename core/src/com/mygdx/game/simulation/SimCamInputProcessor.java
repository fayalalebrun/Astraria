package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Input;
import com.badlogic.gdx.InputProcessor;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Camera_Movement;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.Collections;

import static com.badlogic.gdx.Gdx.input;

/**
 * Created by Fran on 3/17/2018.
 */
public class SimCamInputProcessor implements InputProcessor{

    private Camera camera;

    private int lastY, lastX;

    private boolean rightClickDown;

    private final Vector2f temp;

    private ArrayList<Clickable3DObject> clickable3DObjects;

    private SimulationScreen simulationScreen;

    public SimCamInputProcessor(Camera cam, SimulationScreen simulationScreen) {
        this.camera = cam;
        clickable3DObjects = new ArrayList<Clickable3DObject>();
        temp = new Vector2f();
        this.simulationScreen = simulationScreen;
    }

    public void updateClickableObjects(ArrayList<Clickable3DObject> objects){
        clickable3DObjects.clear();
        clickable3DObjects.addAll(objects);
        Collections.sort(clickable3DObjects);
    }

    @Override
    public boolean keyDown(int keycode) {
        switch (keycode){
            case Input.Keys.W:
                camera.processKeyboard(Camera_Movement.FORWARD, false);
                return true;
            case Input.Keys.S:
                camera.processKeyboard(Camera_Movement.BACKWARD, false);
                return true;
            case Input.Keys.A:
                camera.processKeyboard(Camera_Movement.LEFT, false);
                return true;
            case Input.Keys.D:
                camera.processKeyboard(Camera_Movement.RIGHT, false);
                return true;
            case Input.Keys.Q:
                camera.processKeyboard(Camera_Movement.ROLL_LEFT, false);
                return true;
            case Input.Keys.E:
                camera.processKeyboard(Camera_Movement.ROLL_RIGHT,false);
                return true;
            case Input.Keys.SPACE:
                camera.processKeyboard(Camera_Movement.UP, false);
                return true;
            case Input.Keys.SHIFT_LEFT:
                camera.processKeyboard(Camera_Movement.DOWN, false);
                return true;
            case Input.Keys.UP:
                camera.changeSpeed(10);
                return true;
            case Input.Keys.DOWN:
                camera.changeSpeed(-10);
                return true;
            case Input.Keys.LEFT:
                SimulationScreen.simSpeed/=10;
                return true;
            case Input.Keys.RIGHT:
                SimulationScreen.simSpeed*=10;
                return true;
        }
        return false;
    }

    @Override
    public boolean keyUp(int keycode) {
        switch (keycode){
            case Input.Keys.W:
                camera.processKeyboard(Camera_Movement.FORWARD, true);
                return true;
            case Input.Keys.S:
                camera.processKeyboard(Camera_Movement.BACKWARD, true);
                return true;
            case Input.Keys.A:
                camera.processKeyboard(Camera_Movement.LEFT, true);
                return true;
            case Input.Keys.D:
                camera.processKeyboard(Camera_Movement.RIGHT, true);
                return true;
            case Input.Keys.Q:
                camera.processKeyboard(Camera_Movement.ROLL_LEFT, true);
                return true;
            case Input.Keys.E:
                camera.processKeyboard(Camera_Movement.ROLL_RIGHT, true);
                return true;
            case Input.Keys.SPACE:
                camera.processKeyboard(Camera_Movement.UP, true);
                return true;
            case Input.Keys.SHIFT_LEFT:
                camera.processKeyboard(Camera_Movement.DOWN,true);
                return true;
            case Input.Keys.H:
                simulationScreen.getSimulationScreenInterface().toggleVisible();
                return true;
        }
        return false;
    }

    @Override
    public boolean keyTyped(char character) {
        return false;
    }

    @Override
    public boolean touchDown(int screenX, int screenY, int pointer, int button) {
        if(button == Input.Buttons.RIGHT){
            rightClickDown = true;
            lastX = screenX;
            lastY = screenY;
            return true;
        }
        return false;
    }

    @Override
    public boolean touchUp(int screenX, int screenY, int pointer, int button) {
        if(button == Input.Buttons.LEFT){
            screenY = Gdx.graphics.getHeight() - screenY;
            for(Clickable3DObject object : this.clickable3DObjects){
                temp.set(screenX,screenY);
                temp.sub(object.getScreenCoords());
                if(Math.abs(temp.length())<20){
                    camera.setLock(object.getParent());
                    return true;
                }
            }
        }

        if(button == Input.Buttons.RIGHT){
            rightClickDown = false;
            return true;
        }
        return false;
    }

    @Override
    public boolean touchDragged(int screenX, int screenY, int pointer) {
        if(rightClickDown){
            camera.processMouseMovement(screenX-lastX, lastY-screenY);
            lastX = screenX;
            lastY = screenY;
        }
        return false;
    }

    @Override
    public boolean mouseMoved(int screenX, int screenY) {


        return false;
    }

    @Override
    public boolean scrolled(int amount) {

        camera.changeSpeed(-amount);
        return true;
    }
}
