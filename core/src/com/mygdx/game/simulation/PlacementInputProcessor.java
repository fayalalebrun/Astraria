package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.Input;
import com.badlogic.gdx.InputProcessor;
import com.mygdx.game.simulation.renderer.Camera;

/**
 * Created by fraayala19 on 4/23/18.
 */
public class PlacementInputProcessor implements InputProcessor{
    private Camera camera;
    private PlacementManager placementManager;
    private SimulationScreen simulationScreen;

    public PlacementInputProcessor(Camera camera, PlacementManager placementManager, SimulationScreen simulationScreen) {
        this.camera = camera;
        this.placementManager = placementManager;
        this.simulationScreen = simulationScreen;
    }

    @Override
    public boolean keyDown(int keycode) {
        return false;
    }

    @Override
    public boolean keyUp(int keycode) {
        return false;
    }

    @Override
    public boolean keyTyped(char character) {
        return false;
    }

    @Override
    public boolean touchDown(int screenX, int screenY, int pointer, int button) {
        if(placementManager.isActive()&&button == Input.Buttons.RIGHT){
            placementManager.place(camera, simulationScreen);
            return true;
        }
        return false;
    }

    @Override
    public boolean touchUp(int screenX, int screenY, int pointer, int button) {
        return false;
    }

    @Override
    public boolean touchDragged(int screenX, int screenY, int pointer) {
        return false;
    }

    @Override
    public boolean mouseMoved(int screenX, int screenY) {
        return false;
    }

    @Override
    public boolean scrolled(int amount) {
        return false;
    }
}
