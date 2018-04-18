package com.mygdx.game.simulation.ui;

import com.badlogic.gdx.InputMultiplexer;
import com.badlogic.gdx.scenes.scene2d.Stage;
import com.badlogic.gdx.utils.viewport.ScreenViewport;
import com.kotcrab.vis.ui.widget.Menu;
import com.kotcrab.vis.ui.widget.MenuBar;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationScreen;

/**
 * Created by fraayala19 on 4/18/18.
 */
public class SimulationScreenInterface {
    private SimulationScreen simulationScreen;
    private Stage uiStage;
    private MenuBar menuBar;
    private VisTable menuBarTable;

    public SimulationScreenInterface(SimulationScreen simulationScreen, InputMultiplexer multiplexer) {
        this.simulationScreen = simulationScreen;

        uiStage = new Stage(new ScreenViewport());

        menuBarTable = new VisTable();
        menuBar = new MenuBar();

        uiStage.addActor(menuBarTable);

        menuBarTable.setFillParent(true);
        menuBarTable.left().top();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();


        multiplexer.addProcessor(uiStage);
        uiStage.addActor(new VisWindow("test"));

        menuBar.addMenu(new Menu("Test"));
    }

    public void update(){
        uiStage.act();
    }

    public void render(){
        uiStage.draw();
    }

    public void resize(int width, int height){
        uiStage.getViewport().update(width,height, true);

        menuBarTable.clearChildren();
        menuBarTable.add(menuBar.getTable()).fillX().expandX().row();
    }
}
