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

    private final VisValidatableTextField yPosField;
    private final VisTextButton yPosButton;

    private final VisValidatableTextField zPosField;
    private final VisTextButton zPosButton;

    private final VisValidatableTextField xVelField;
    private final VisTextButton xVelButton;

    private final VisValidatableTextField yVelField;
    private final VisTextButton yVelButton;

    private final VisValidatableTextField zVelField;
    private final VisTextButton zVelButton;

    public SimObjectTracker(final SimulationObject simulationObject) {
        this.simulationObject = simulationObject;
        formatter = new DecimalFormat("#.##E0");

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
                    simulationObject.setMass(Double.parseDouble(massField.getText()));
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
                    simulationObject.setXPos(Units.AUToM(Double.parseDouble(xPosField.getText())));
                }
            }
        });

        initializeNum(xPosField, xPosButton);

        yPosField = new VisValidatableTextField();
        yPosButton = new VisTextButton("Change");

        yPosButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!yPosField.isDisabled()){
                    simulationObject.setYPos(Units.AUToM(Double.parseDouble(yPosField.getText())));
                }
            }
        });

        initializeNum(yPosField, yPosButton);

        zPosField = new VisValidatableTextField();
        zPosButton = new VisTextButton("Change");

        zPosButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!zPosField.isDisabled()){
                    simulationObject.setZPos(Units.AUToM(Double.parseDouble(zPosField.getText())));
                }
            }
        });

        initializeNum(zPosField, zPosButton);

        xVelField = new VisValidatableTextField();
        xVelButton = new VisTextButton("Change");

        xVelButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!xVelField.isDisabled()){
                    simulationObject.setXVel(Units.kmToM(Double.parseDouble(xVelField.getText())));
                }
            }
        });

        initializeNum(xVelField, xVelButton);

        yVelField = new VisValidatableTextField();
        yVelButton = new VisTextButton("Change");

        yVelButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!yVelField.isDisabled()){
                    simulationObject.setYVel(Units.kmToM(Double.parseDouble(yVelField.getText())));
                }
            }
        });

        initializeNum(yVelField, yVelButton);

        zVelField = new VisValidatableTextField();
        zVelButton = new VisTextButton("Change");

        zVelButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(!zVelField.isDisabled()){
                    simulationObject.setZVel(Units.kmToM(Double.parseDouble(zVelField.getText())));
                }
            }
        });

        initializeNum(zVelField, zVelButton);
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

        addField(table, "y-pos(au): ", yPosField, yPosButton);
        table.row();

        addField(table, "z-pos(au): ", zPosField, zPosButton);
        table.row();

        addField(table, "x-vel(km/s): ", xVelField, xVelButton);
        table.row();

        addField(table, "y-vel(km/s): ", yVelField, yVelButton);
        table.row();

        addField(table, "z-vel(km/s): ", zVelField, zVelButton);
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
            massField.setText(formatter.format(simulationObject.getMass()));
        }

        if(xPosField.isDisabled()){
            xPosField.setText(formatter.format(Units.mToAU(simulationObject.getXPos())));
        }

        if(yPosField.isDisabled()){
            yPosField.setText(formatter.format(Units.mToAU(simulationObject.getYPos())));
        }

        if(zPosField.isDisabled()){
            zPosField.setText(formatter.format(Units.mToAU(simulationObject.getZPos())));
        }

        if(xVelField.isDisabled()){
            xVelField.setText(formatter.format(Units.mToKM(simulationObject.getXVel())));
        }

        if(yVelField.isDisabled()){
            yVelField.setText(formatter.format(Units.mToKM(simulationObject.getYVel())));
        }

        if(zVelField.isDisabled()){
            zVelField.setText(formatter.format(Units.mToKM(simulationObject.getZVel())));
        }
    }
}
