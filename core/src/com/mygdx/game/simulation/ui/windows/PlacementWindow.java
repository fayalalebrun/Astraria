package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.g2d.TextureRegion;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.ui.Image;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.scenes.scene2d.utils.Drawable;
import com.badlogic.gdx.scenes.scene2d.utils.TextureRegionDrawable;
import com.badlogic.gdx.utils.Array;
import com.kotcrab.vis.ui.VisUI;
import com.kotcrab.vis.ui.util.dialog.Dialogs;
import com.kotcrab.vis.ui.util.form.SimpleFormValidator;
import com.kotcrab.vis.ui.widget.*;
import com.kotcrab.vis.ui.widget.color.ColorPicker;
import com.kotcrab.vis.ui.widget.color.ColorPickerAdapter;
import com.mygdx.game.Boot;
import com.mygdx.game.simulation.*;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.*;

import java.util.ArrayList;
import java.util.Arrays;

/**
 * Created by Fran on 4/22/2018.
 */
public class PlacementWindow extends VisWindow{

    private static final Drawable white = VisUI.getSkin().getDrawable("white");

    private final VisSelectBox<String> typeSelect = new VisSelectBox<String>();

    private final VisLabel typeLabel = new VisLabel("Object type: ");
    private final VisLabel nameLabel = new VisLabel("Name: ");
    private final VisLabel radiusLabel = new VisLabel("Radius(km): ");
    private final VisLabel massLabel = new VisLabel("Mass(kg): ");
    private final VisLabel orbitColorLabel = new VisLabel("Orbit color: ");
    private final VisLabel textureLabel = new VisLabel("Texture: ");
    private final VisLabel inclinationLabel = new VisLabel("Inclination(deg): ");
    private final VisLabel axisRightLabel = new VisLabel("Axis Right Ascension(deg): ");
    private final VisLabel rotPeriodLabel = new VisLabel("Rotation Period(deg/day): ");
    private final VisLabel temperatureLabel = new VisLabel("Temperature: ");
    private final VisLabel atmoColorLabel = new VisLabel("Atmosphere color: ");


    private final VisValidatableTextField nameField = new VisValidatableTextField();
    private final VisValidatableTextField radiusField = new VisValidatableTextField();
    private final VisValidatableTextField massField = new VisValidatableTextField();
    private final VisSelectBox<GLTexture> textureSelect = new VisSelectBox<GLTexture>();
    private final VisTextButton orbitColorButton = new VisTextButton("Choose color");
    private final VisValidatableTextField inclinationField = new VisValidatableTextField();
    private final VisValidatableTextField axisRightField = new VisValidatableTextField();
    private final VisValidatableTextField rotPeriodField = new VisValidatableTextField();
    private final VisValidatableTextField temperatureField = new VisValidatableTextField();
    private final VisTextButton atmoColorButton = new VisTextButton("Choose color");

    private final Image orbitColImage = new Image(white);
    private final Image atmoColImage = new Image(white);
    private String colSelect = "";

    private final VisTextButton baseTracker = new VisTextButton("");
    private final VisTextButton rotTracker = new VisTextButton("");
    private final VisTextButton tempTracker = new VisTextButton("");

    private final VisTable fieldsTable = new VisTable(true);

    private final VisImageButton launchButton = new VisImageButton(
            convertToDrawable(Boot.manager.get("icons/target.png", Texture.class)), "Launch Tool");

    private final LaunchToolWindow launchToolWindow;


    private final ColorPicker picker = new ColorPicker("", new ColorPickerAdapter(){
        @Override
        public void finished (Color newColor) {
            if(getColSelect().equals("orbit")){
                orbitColImage.setColor(newColor);
            } else if (getColSelect().equals("atmo")){
                atmoColImage.setColor(newColor);
            }
        }
    });

    private Renderer renderer;

    private PlacementManager placementManager;

    public PlacementWindow(LaunchToolWindow launchToolWindow, Renderer renderer, PlacementManager placementManager) {
        super("New Object Placement");
        this.launchToolWindow = launchToolWindow;
        this.placementManager = placementManager;
        launchToolWindow.setPlacementWindow(this);
        this.renderer = renderer;

        add(fieldsTable);
        createWidgets();
        addPlanetFields(fieldsTable);
        createValidators();
        row();
        add(launchButton).padTop(3f).left();
        pack();
    }

    private void createWidgets(){
        typeSelect.setItems("Planet", "Atmosphere Planet","Star", "Black Hole");
        typeSelect.setSelected("Planet");

        typeSelect.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(typeSelect.getSelected().equals("Planet")){
                    fieldsTable.clear();
                    addPlanetFields(fieldsTable);
                    pack();
                } else if(typeSelect.getSelected().equals("Atmosphere Planet")){
                    fieldsTable.clear();
                    addPlanetAtmoFields(fieldsTable);
                    pack();
                } else if (typeSelect.getSelected().equals("Star")){
                    fieldsTable.clear();
                    addStarFields(fieldsTable);
                    pack();
                } else if(typeSelect.getSelected().equals("Black Hole")){
                    fieldsTable.clear();
                    addBlackHoleFields(fieldsTable);
                    pack();
                }
            }
        });

        textureSelect.setItems(Arrays.copyOf(Warehouse.getOpenGLTextureManager().getLoadedTextures().toArray(),
                Warehouse.getOpenGLTextureManager().getLoadedTextures().toArray().length,GLTexture[].class));
        textureSelect.setSelectedIndex(0);

        orbitColorButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                picker.setColor(orbitColImage.getColor());
                picker.setName("Select orbit color");
                setColSelect("orbit");
                getStage().addActor(picker.fadeIn());
            }
        });

        atmoColorButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                picker.setColor(atmoColImage.getColor());
                picker.setName("Select atmosphere color");
                setColSelect("atmo");
                getStage().addActor(picker.fadeIn());
            }
        });

        launchButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(checkIfInputValid()){
                    SimulationObject simObj = null;
                    Body body = new Body(Double.parseDouble(massField.getText()),0,0,0,0,0,0);
                    if(typeSelect.getSelected().equals("Planet")){
                        simObj = new Planet(renderer,textureSelect.getSelected(),Float.parseFloat(radiusField.getText()),nameField.getText(),
                                renderer.getTransformation(), body,orbitColImage.getColor());
                        simObj.setRotationParameters(getRadParam(inclinationField.getText()),getRadParam(axisRightField.getText()),
                                getRadParam(rotPeriodField.getText()),0f);
                    } else if (typeSelect.getSelected().equals("Atmosphere Planet")){
                        simObj = new AtmospherePlanet(renderer, textureSelect.getSelected(),Float.parseFloat(radiusField.getText()),
                                nameField.getText(), orbitColImage.getColor(),renderer.getTransformation(), body,
                                renderer.getLightSourceManager(), atmoColImage.getColor(),null);
                        simObj.setRotationParameters(getRadParam(inclinationField.getText()),getRadParam(axisRightField.getText()),
                                getRadParam(rotPeriodField.getText()),0f);
                    } else if(typeSelect.getSelected().equals("Black Hole")){
                        Sphere s = new Sphere(renderer.getTransformation(),
                                null,false);
                        simObj = new BlackHole(s,renderer.getBlackHoleShader(),renderer.getLineShader(),
                                Float.parseFloat(radiusField.getText()),nameField.getText(),renderer.getTransformation(),
                                body, orbitColImage.getColor(),renderer.getSkybox());
                    } else if(typeSelect.getSelected().equals("Star")){
                        simObj = new Star(renderer,textureSelect.getSelected(),Float.parseFloat(radiusField.getText()),
                                nameField.getText(), renderer.getTransformation(), body,orbitColImage.getColor(),
                                Float.parseFloat(temperatureField.getText()));
                        simObj.setRotationParameters(getRadParam(inclinationField.getText()),getRadParam(axisRightField.getText()),
                                getRadParam(rotPeriodField.getText()),0f);
                    }

                    launchToolWindow.setSimulationObject(simObj);
                    launchToolWindow.setVisible(true);
                    setVisible(false);

                } else {
                    Dialogs.showErrorDialog(getStage(), "Parameters incorrect");
                }
            }
        });
    }

    private float getRadParam(String param){
        return (float)Math.toRadians(Float.parseFloat(param));
    }

    private void addGeneral(VisTable table){
        table.add(typeLabel).width(150f);
        table.add(typeSelect);
        table.row();

        addField(table, nameLabel, nameField);
        table.row();

        addField(table, radiusLabel, radiusField);
        table.row();

        addField(table, massLabel, massField);
        table.row();

        table.add(orbitColorLabel).left();
        table.add(orbitColorButton);
        table.add(orbitColImage).size(24f);
        table.row();
    }

    private void addTexAndRot(VisTable table){
        table.add(textureLabel).width(150);
        table.add(textureSelect);
        table.row();

        addField(table, inclinationLabel, inclinationField);
        table.row();

        addField(table, axisRightLabel, axisRightField);
        table.row();

        addField(table, rotPeriodLabel, rotPeriodField);
        table.row();
    }

    private void addPlanetFields(VisTable table){
        addGeneral(table);
        addTexAndRot(table);
    }

    private void addStarFields(VisTable table){
        addGeneral(table);
        addTexAndRot(table);

        addField(table, temperatureLabel, temperatureField);
        table.row();
    }

    private void addPlanetAtmoFields(VisTable table){
        addGeneral(table);
        addTexAndRot(table);

        table.add(atmoColorLabel).left();
        table.add(atmoColorButton);
        table.add(atmoColImage).size(24f);
        table.row();
    }

    private void addBlackHoleFields(VisTable table){
        addGeneral(table);
    }

    private void createValidators(){
        SimpleFormValidator baseValidator = new SimpleFormValidator(baseTracker);
        baseValidator.notEmpty(nameField,"");

        baseValidator.notEmpty(radiusField,"");
        baseValidator.floatNumber(radiusField,"");

        baseValidator.notEmpty(massField,"");
        baseValidator.floatNumber(massField,"");

        SimpleFormValidator rotValidator = new SimpleFormValidator(rotTracker);

        rotValidator.notEmpty(inclinationField, "");
        rotValidator.floatNumber(inclinationField, "");

        rotValidator.notEmpty(axisRightField, "");
        rotValidator.floatNumber(axisRightField, "");

        rotValidator.notEmpty(rotPeriodField, "");
        rotValidator.floatNumber(rotPeriodField, "");

        SimpleFormValidator tempValidator = new SimpleFormValidator(tempTracker);

        tempValidator.notEmpty(temperatureField, "");
        tempValidator.floatNumber(temperatureField, "");
    }

    private void addField(VisTable table, VisLabel label, VisValidatableTextField field){
        table.add(label).width(150f);
        table.add(field).width(150f);
    }

    private boolean checkIfInputValid(){
        if(typeSelect.getSelected().equals("Planet")){
            return !baseTracker.isDisabled()&&!rotTracker.isDisabled();
        } else if(typeSelect.getSelected().equals("Atmosphere Planet")){
            return !baseTracker.isDisabled()&&!rotTracker.isDisabled();
        } else if(typeSelect.getSelected().equals("Star")){
            return !baseTracker.isDisabled()&&!rotTracker.isDisabled()&&!tempTracker.isDisabled();
        } else if(typeSelect.getSelected().equals("Black Hole")){
            return !baseTracker.isDisabled();
        }
        return false;
    }

    @Override
    public void act(float delta) {
        super.act(delta);
    }

    public String getColSelect() {
        return colSelect;
    }


    public void setColSelect(String colSelect) {
        this.colSelect = colSelect;
    }

    private TextureRegionDrawable convertToDrawable(Texture texture){
        return new TextureRegionDrawable(new TextureRegion(texture));
    }
}
