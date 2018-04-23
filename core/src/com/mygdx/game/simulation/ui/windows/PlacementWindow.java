package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.ui.Image;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.scenes.scene2d.utils.Drawable;
import com.badlogic.gdx.utils.Array;
import com.kotcrab.vis.ui.VisUI;
import com.kotcrab.vis.ui.widget.*;
import com.kotcrab.vis.ui.widget.color.ColorPicker;
import com.kotcrab.vis.ui.widget.color.ColorPickerAdapter;
import com.mygdx.game.simulation.renderer.GLTexture;
import com.mygdx.game.simulation.renderer.Warehouse;

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

    private final VisLabel errorLabel = new VisLabel("Please enter parameters");

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

    private final VisTable fieldsTable = new VisTable(true);

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

    public PlacementWindow() {
        super("New Object Placement");
        add(fieldsTable);
        createWidgets();
        addPlanetFields(fieldsTable);
        row();
        add(errorLabel);
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

        errorLabel.setColor(Color.RED);
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

    private void addField(VisTable table, VisLabel label, VisValidatableTextField field){
        table.add(label).width(150f);
        table.add(field).width(150f);
    }

    public String getColSelect() {
        return colSelect;
    }


    public void setColSelect(String colSelect) {
        this.colSelect = colSelect;
    }
}
