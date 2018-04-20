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

    private final VisLabel nameLabel;
    private VisValidatableTextField nameField;
    private VisTextButton nameSetButton;
    private SimpleFormValidator nameValidator;

    private final VisLabel radiusLabel;
    private VisValidatableTextField radiusField;
    private VisTextButton radiusSetButton;
    private SimpleFormValidator radiusValidator;

    private final VisLabel massLabel;
    private VisValidatableTextField massField;
    private VisTextButton massSetButton;
    private SimpleFormValidator massValidator;

    private final VisLabel xPosLabel;
    private VisValidatableTextField xPosField;
    private VisTextButton xPosButton;
    private SimpleFormValidator xPosValidator;

    private final VisLabel yPosLabel;
    private VisValidatableTextField yPosField;
    private VisTextButton yPosButton;
    private SimpleFormValidator yPosValidator;

    private final VisLabel zPosLabel;
    private VisValidatableTextField zPosField;
    private VisTextButton zPosButton;
    private SimpleFormValidator zPosValidator;

    private final VisLabel xVelLabel;
    private VisValidatableTextField xVelField;
    private VisTextButton xVelButton;
    private SimpleFormValidator xVelValidator;


    public SimObjectTracker(final SimulationObject simulationObject) {
        this.simulationObject = simulationObject;
        formatter = new DecimalFormat("##E0");

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

        massLabel = new VisLabel();
        massField = new VisValidatableTextField();
        massSetButton = new VisTextButton("Set");
        massValidator = new SimpleFormValidator(massSetButton);
        massValidator.notEmpty(massField,"");
        massValidator.floatNumber(massField,"");
        massSetButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationObject.setMass(Double.parseDouble(massField.getText()));
            }
        });

        xPosLabel = new VisLabel();
        xPosField = new VisValidatableTextField();
        xPosButton = new VisTextButton("Set");
        xPosValidator = new SimpleFormValidator(xPosButton);
        xPosValidator.notEmpty(xPosField, "");
        xPosValidator.floatNumber(xPosField, "");
        xPosButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationObject.setXPos(Units.AUToM(Double.parseDouble(xPosField.getText())));
            }
        });

        yPosLabel = new VisLabel();
        yPosField = new VisValidatableTextField();
        yPosButton = new VisTextButton("Set");
        yPosValidator = new SimpleFormValidator(yPosButton);
        yPosValidator.notEmpty(yPosField, "");
        yPosValidator.floatNumber(yPosField,"");
        yPosButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationObject.setYPos(Units.AUToM(Double.parseDouble(yPosField.getText())));
            }
        });

        zPosLabel = new VisLabel();
        zPosField = new VisValidatableTextField();
        zPosButton = new VisTextButton("Set");
        zPosValidator = new SimpleFormValidator(zPosButton);
        zPosValidator.notEmpty(zPosField,"");
        zPosValidator.floatNumber(zPosField,"");
        zPosButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationObject.setZPos(Units.AUToM(Double.parseDouble(zPosField.getText())));
            }
        });

        xVelLabel = new VisLabel();
        xVelField = new VisValidatableTextField();
        xVelButton = new VisTextButton("Set");
        xVelValidator = new SimpleFormValidator(xVelButton);
        xVelValidator.notEmpty(xVelField,"");
        xVelValidator.floatNumber(xVelField,"");
        xVelButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationObject.setXVel(Units.kmToM(Double.parseDouble(xVelField.getText())));
            }
        });
    }


    public void addToTable(VisTable table){

        addField(table, nameLabel, nameField, nameSetButton);
        table.row();

        addField(table, radiusLabel, radiusField, radiusSetButton);
        table.row();

        addField(table, massLabel, massField, massSetButton);
        table.row();

        addField(table,xPosLabel, xPosField, xPosButton);
        table.row();

        addField(table,yPosLabel,yPosField,yPosButton);
        table.row();

        addField(table, zPosLabel, zPosField, zPosButton);
        table.row();

        addField(table, xVelLabel, xVelField, xVelButton);
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
        massLabel.setText("Mass(kg): "+formatter.format(simulationObject.getMass()));
        xPosLabel.setText("x-pos(au): "+formatter.format(Units.mToAU(simulationObject.getXPos())));
        yPosLabel.setText("y-pos(au): "+formatter.format(Units.mToAU(simulationObject.getYPos())));
        zPosLabel.setText("z-pos(au): "+formatter.format(Units.mToAU(simulationObject.getZPos())));
        xVelLabel.setText("x-vel(km/s): "+formatter.format(Units.mToKM(simulationObject.getXVel())));
    }
}
