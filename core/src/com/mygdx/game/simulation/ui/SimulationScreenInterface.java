package com.mygdx.game.simulation.ui;

import com.badlogic.gdx.InputMultiplexer;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.InputEvent;
import com.badlogic.gdx.scenes.scene2d.InputListener;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.scenes.scene2d.ui.TextField;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.kotcrab.vis.ui.widget.Menu;
import com.kotcrab.vis.ui.widget.MenuBar;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.ui.windows.ObjectListWindow;

/**
 * Created by fraayala19 on 4/18/18.
 */
public class SimulationScreenInterface {
    private SimulationScreen simulationScreen;
    private Stage uiStage;
    private MenuBar menuBar;
    private VisTable menuBarTable;

    private ObjectListWindow objectListWindow;

    public SimulationScreenInterface(SimulationScreen simulationScreen, InputMultiplexer multiplexer) {
        this.simulationScreen = simulationScreen;

        uiStage = new Stage(new ScreenViewport());

        uiStage.getRoot().addCaptureListener(new InputListener() {
            public boolean touchDown(InputEvent event, float x, float y, int pointer, int button) {

                if (uiStage.hit(x,y,true) == null) {
                    uiStage.setKeyboardFocus(null);
                    uiStage.setScrollFocus(null);
                }
                return false;
            }
        });


        menuBarTable = new VisTable();
        menuBar = new MenuBar();

        uiStage.addActor(menuBarTable);

        menuBarTable.setFillParent(true);
        menuBarTable.left().top();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();


        multiplexer.addProcessor(uiStage);


        addWindows();
        positionWindows();
    }

    private void addWindows() {
        objectListWindow = new ObjectListWindow(simulationScreen);
        uiStage.addActor(objectListWindow);
    }

    private void positionWindows() {
        objectListWindow.setPosition(0, 0);
    }

    public void addObject(SimulationObject simulationObject) {
        objectListWindow.addObject(simulationObject);
    }

    public void removeObject(SimulationObject simulationObject) {
        objectListWindow.removeobject(simulationObject);
    }

    public void update() {
        uiStage.act();
    }

    public void render() {
        uiStage.draw();
    }

    public void resize(int width, int height) {
        uiStage.getViewport().update(width, height, true);

        menuBarTable.clearChildren();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();
    }
}
