package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisScrollPane;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.ui.stat_trackers.SimObjectTracker;

/**
 * Created by fraayala19 on 4/19/18.
 */
public class StatsWindow extends VisWindow{

    SimObjectTracker simObjectTracker = null;

    VisTable mainTable;

    VisTextButton deleteButton;

    private SimulationScreen simulationScreen;

    public StatsWindow(SimulationScreen simulationScreen) {
        super("Object Statistics");
        this.simulationScreen = simulationScreen;
        TableUtils.setSpacingDefaults(this);
        mainTable = new VisTable();
        TableUtils.setSpacingDefaults(mainTable);
        addWidgets();
        setSize(285,285);
    }

    private void addWidgets(){
        VisScrollPane scrollPane = new VisScrollPane(mainTable);
        scrollPane.setFadeScrollBars(false);
        scrollPane.setFlickScroll(false);

        add(scrollPane).spaceTop(30).growX().width(280).height(220f);
        row();

        deleteButton = new VisTextButton("Delete");

        deleteButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(getSimObjectTracker()!=null){
                    simulationScreen.removeObject(getSimObjectTracker().getSimulationObject());
                }
            }
        });

        add(deleteButton).right();
        row();
    }

    public void setSimObjectTracker(SimObjectTracker simObjectTracker){
        this.simObjectTracker = simObjectTracker;
        mainTable.clear();
        simObjectTracker.addToTable(mainTable);
    }

    public void update(){
        if(simObjectTracker!=null) {
            this.simObjectTracker.update();
        }
    }

    public SimObjectTracker getSimObjectTracker() {
        return simObjectTracker;
    }
}
