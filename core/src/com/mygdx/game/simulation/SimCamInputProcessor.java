package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Input;
import com.badlogic.gdx.InputProcessor;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.renderer.Camera_Movement;

import static com.badlogic.gdx.Gdx.input;

/**
 * Created by Fran on 3/17/2018.
 */
public class SimCamInputProcessor implements InputProcessor{

    private Camera camera;

    private int lastY, lastX;

    private boolean firstMove, rightClickDown;

    public SimCamInputProcessor(Camera cam) {
        this.camera = cam;
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
                System.out.println("Simspeed: "+SimulationScreen.simSpeed);
                return true;
            case Input.Keys.RIGHT:
                SimulationScreen.simSpeed*=10;
                System.out.println("Simspeed: "+SimulationScreen.simSpeed);
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
        return false;
    }
}
