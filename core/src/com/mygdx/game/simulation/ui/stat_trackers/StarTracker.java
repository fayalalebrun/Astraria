package com.mygdx.game.simulation.ui.stat_trackers;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisValidatableTextField;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.Star;

/**
 * Created by fraayala19 on 5/3/18.
 */
public class StarTracker extends SimObjectTracker{

    private final VisValidatableTextField tempField;
    private final VisTextButton tempButton;

    public StarTracker(final SimulationObject simulationObject) {
        super(simulationObject);

        tempField = new VisValidatableTextField();
        tempButton = new VisTextButton("Change");

        tempButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!tempField.isDisabled()){
                    ((Star)simulationObject).setTemperature(Float.parseFloat(tempField.getText()));
                }
            }
        });

        initializeNum(tempField,tempButton);
    }

    @Override
    public void addToTable(VisTable table) {
        super.addToTable(table);

        addField(table,"Temperature(K): ",tempField,tempButton);
        table.row();
    }

    @Override
    public void update() {
        super.update();

        if(tempField.isDisabled()){
            tempField.setText(formatter.format(((Star)getSimulationObject()).getTemperature()));
        }
    }
}
