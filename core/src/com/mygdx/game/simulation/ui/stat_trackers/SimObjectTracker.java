package com.mygdx.game.simulation.ui.stat_trackers;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.ui.ScrollPane;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.form.SimpleFormValidator;
import com.kotcrab.vis.ui.widget.*;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.ui.SimulationScreenInterface;
import com.mygdx.game.simulation.ui.windows.StatsWindow;

/**
 * Created by fraayala19 on 4/19/18.
 */
public class SimObjectTracker {
    private final SimulationObject simulationObject;

    private final VisLabel nameLabel;
    private VisValidatableTextField nameField;
    private VisTextButton nameSetButton;
    private SimpleFormValidator nameValidator;

    private final VisLabel radiusLabel;
    private VisValidatableTextField radiusField;
    private VisTextButton radiusSetButton;
    private SimpleFormValidator radiusValidator;



    public SimObjectTracker(final SimulationObject simulationObject) {
        this.simulationObject = simulationObject;


        nameLabel = new VisLabel();
        nameField = new VisValidatableTextField();
        nameSetButton = new VisTextButton("Set");
        nameValidator = new SimpleFormValidator(nameSetButton);
        nameValidator.notEmpty(nameField,"Name empty");
        nameSetButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationObject.setName(nameField.getText());
            }
        });

        radiusLabel = new VisLabel();
        radiusField = new VisValidatableTextField();
        radiusSetButton = new VisTextButton("Set");
        radiusValidator = new SimpleFormValidator(radiusSetButton);
        radiusValidator.notEmpty(radiusField,"");
        radiusValidator.floatNumber(radiusField,"");
        radiusSetButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationObject.setRadius(Float.parseFloat(radiusField.getText()));
            }
        });

    }


    public void addToTable(VisTable table){

        addField(table, nameLabel, nameField, nameSetButton);
        table.row();

        addField(table, radiusLabel, radiusField, radiusSetButton);
        table.row();
    }

    protected void addField(VisTable table, VisLabel label, VisValidatableTextField field, VisTextButton button){
        table.padLeft(3f);
        table.add(wrapInPane(label)).width(120);
        table.add(field).width(100).padRight(3f);
        table.add(button).fill().padRight(8f);
    }

    protected ScrollPane wrapInPane(VisLabel label){
        VisScrollPane scrollPane = new VisScrollPane(label);
        scrollPane.setStyle(new ScrollPane.ScrollPaneStyle(null, null, null, null, null));
        scrollPane.setupOverscroll(0f,0f,0f);

        return scrollPane;
    }

    public void update(){
        nameLabel.setText("Name: "+simulationObject.getName());
        radiusLabel.setText("Radius(km): "+simulationObject.getRadius());
    }
}
