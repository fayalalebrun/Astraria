package com.mygdx.game.simulation.ui;

import com.badlogic.gdx.InputMultiplexer;
import com.badlogic.gdx.scenes.scene2d.*;
import com.badlogic.gdx.scenes.scene2d.ui.TextField;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.kotcrab.vis.ui.widget.Menu;
import com.kotcrab.vis.ui.widget.MenuBar;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.logic.algorithms.NBodyAlgorithm;
import com.mygdx.game.simulation.ui.windows.*;
import javafx.scene.control.ToolBar;

/**
 * Created by fraayala19 on 4/18/18.
 */
public class SimulationScreenInterface {
    private SimulationScreen simulationScreen;
    private Stage uiStage;
    private MenuBar menuBar;
    private VisTable menuBarTable;

    private ObjectListWindow objectListWindow;
    private StatsWindow statsWindow;
    private ToolbarWindow toolbar;
    private SimInfoWindow simInfoWindow;
    private SimSpeedWindow simSpeedWindow;
    private PlacementWindow placementWindow;

    private Group listGroup;
    private Group infoGroup;
    private Group simSpeedGroup;

    public SimulationScreenInterface(SimulationScreen simulationScreen, InputMultiplexer multiplexer, NBodyAlgorithm nBodyAlgorithm) {
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

        this.simInfoWindow = new SimInfoWindow(nBodyAlgorithm);

        menuBarTable = new VisTable();
        menuBar = new MenuBar();

        uiStage.addActor(menuBarTable);

        menuBarTable.setFillParent(true);
        menuBarTable.left().top();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();

        listGroup = new Group();
        infoGroup = new Group();
        simSpeedGroup = new Group();

        multiplexer.addProcessor(uiStage);


        addWindows();
        positionWindows();
    }

    private void addWindows() {
        statsWindow = new StatsWindow();
        listGroup.addActor(statsWindow);
        objectListWindow = new ObjectListWindow(simulationScreen, statsWindow);
        listGroup.addActor(objectListWindow);

        infoGroup.addActor(simInfoWindow);

        simSpeedWindow = new SimSpeedWindow();
        simSpeedGroup.addActor(simSpeedWindow);

        placementWindow = new PlacementWindow();

        toolbar = new ToolbarWindow(listGroup, infoGroup, simSpeedGroup);

        uiStage.addActor(listGroup);
        uiStage.addActor(infoGroup);
        uiStage.addActor(simSpeedGroup);
        uiStage.addActor(placementWindow);
        uiStage.addActor(toolbar);
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
        statsWindow.update();
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
