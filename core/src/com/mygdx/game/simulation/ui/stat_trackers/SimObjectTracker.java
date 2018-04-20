package com.mygdx.game.simulation.ui.stat_trackers;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.ui.ScrollPane;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.form.SimpleFormValidator;
import com.kotcrab.vis.ui.widget.*;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.logic.helpers.Units;
import com.mygdx.game.simulation.ui.SimulationScreenInterface;
import com.mygdx.game.simulation.ui.windows.StatsWindow;
import sun.java2d.pipe.SpanShapeRenderer;

import java.text.DecimalFormat;

/**
 * Created by fraayala19 on 4/19/18.
 */
public class SimObjectTracker {
    private final SimulationObject simulationObject;

    protected DecimalFormat formatter;

    private final VisValidatableTextField nameField;
    private final VisTextButton nameSetButton;

    private final VisValidatableTextField radiusField;
    private final VisTextButton radiusButton;

    private final VisValidatableTextField massField;
    private final VisTextButton massButton;

    private final VisValidatableTextField xPosField;
    private final VisTextButton xPosButton;

    public SimObjectTracker(final SimulationObject simulationObject) {
        this.simulationObject = simulationObject;
        formatter = new DecimalFormat("##E0");

        nameField = new VisValidatableTextField();
        nameSetButton = new VisTextButton("Change");

        nameSetButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(nameField.isDisabled()){
                    nameSetButton.setText("Set");
                } else {
                    simulationObject.setName(nameField.getText());
                    nameSetButton.setText("Change");
                }
                nameField.setDisabled(nameField.isDisabled());
            }
        });

        initialize(nameField,nameSetButton);



        radiusField = new VisValidatableTextField();
        radiusButton = new VisTextButton("Change");

        radiusButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!radiusField.isDisabled()) {
                    simulationObject.setRadius(Float.parseFloat(radiusField.getText()));
                }
            }
        });

        initializeNum(radiusField,radiusButton);



        massField = new VisValidatableTextField();
        massButton = new VisTextButton("Change");

        massButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!massField.isDisabled()){
                    simulationObject.setMass(Float.parseFloat(massField.getText()));
                }
            }
        });

        initializeNum(massField,massButton);

        xPosField = new VisValidatableTextField();
        xPosButton = new VisTextButton("Change");

        xPosButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!xPosField.isDisabled()){
                    simulationObject.setXPos(Units.AUToM(Float.parseFloat(xPosField.getText())));
                }
            }
        });

        initializeNum(xPosField, xPosButton);
    }

    protected SimpleFormValidator initialize(final VisValidatableTextField textField, final VisTextButton textButton){
        SimpleFormValidator validator = new SimpleFormValidator(textButton);
        textField.setDisabled(true);
        validator.notEmpty(textField,"");
        textButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(textField.isDisabled()){
                    textButton.setText("Set");
                } else {
                    textButton.setText("Change");
                }
                textField.setDisabled(!textField.isDisabled());
            }
        });
        return validator;
    }

    protected void initializeNum(VisValidatableTextField field, VisTextButton button){
        initialize(field, button).floatNumber(field, "");
    }

    public void addToTable(VisTable table){
        addField(table, "Name: ",nameField, nameSetButton);
        table.row();

        addField(table, "Radius(km): ", radiusField, radiusButton);
        table.row();

        addField(table, "Mass(kg): ", massField, massButton);
        table.row();

        addField(table, "x-pos(au): ", xPosField, xPosButton);
        table.row();

        
    }

    protected void addField(VisTable table, String tag,VisValidatableTextField field, VisTextButton button){
        table.padLeft(3f);
        table.add(new VisLabel(tag)).width(50);
        table.add(field).width(100);
        table.add(button).width(60f).padRight(8f);
    }

    protected ScrollPane wrapInPane(VisLabel label){
        VisScrollPane scrollPane = new VisScrollPane(label);
        scrollPane.setStyle(new ScrollPane.ScrollPaneStyle(null, null, null, null, null));
        scrollPane.setupOverscroll(0f,0f,0f);

        return scrollPane;
    }

    public void update(){
        if(nameField.isDisabled()){
            nameField.setText(simulationObject.getName());
        }

        if(radiusField.isDisabled()){
            radiusField.setText(simulationObject.getRadius()+"");
        }

        if(massField.isDisabled()){
            massField.setText(simulationObject.getMass()+"");
        }

        if(xPosField.isDisabled()){
            xPosField.setText(Units.mToAU(simulationObject.getXPos())+"");
        }
    }
}
